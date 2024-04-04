use crate::{
    error::ParserError,
    expr::Expr,
    stmt::Stmt,
    token::{ Token, TokenType },
    value::LiteralType
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e)
            }
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![&TokenType::Fun]) {
            match self.function("function") {
                Ok(v) => Ok(v),
                Err(_) => {
                    self.synchronize();
                    Ok(
                        Stmt::Expression {
                            expression: Expr::Literal {
                                value: LiteralType::Nil
                            }
                        }
                    )
                }
            }
        } else if self.match_token(vec![&TokenType::Var]) {
            match self.var_declaration() {
                Ok(v) => Ok(v),
                Err(_) => {
                    self.synchronize();
                    Ok(
                        Stmt::Expression {
                            expression: Expr::Literal {
                                value: LiteralType::Nil
                            }
                        }
                    )
                }
            }
        } else {
            self.statement()
        }
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParserError> {
        let name = match 
            self.consume(
                TokenType::Identifier,
                format!(
                    "Expected{}Name",
                    kind.chars().next().unwrap().to_uppercase().collect::<String>() + &kind[1..]
                ).as_str()
            )
        {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        if let Err(e) = self.consume(
            TokenType::LParen,
            format!(
                "ExpectedLParenAfter{}Name",
                kind.chars().next().unwrap().to_uppercase().collect::<String>() + &kind[1..]
            ).as_str()
        ) {
            return Err(e)
        };

        let mut params: Vec<Token> = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                if params.len() >= 255 {
                    let token = self.peek();
                    return Err(
                        ParserError::TooManyParameters{
                            start: token.start,
                            end: token.end,
                            line: token.line
                        }
                    );
                }

                let parameter = match self.consume(TokenType::Identifier, "ExpectedParameterName") {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
                params.push(parameter);

                if !self.match_token(vec![&TokenType::Comma]) { break };
            }
        }

        if let Err(e) = self.consume(TokenType::RParen, "ExpectedRParenAfterParameters") {
            return Err(e)
        };

        if let Err(e) = self.consume(TokenType::LBrace, "ExpectedRParenAfterParameters") {
            return Err(e)
        };

        let body = match self.block() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        Ok(Stmt::Function{ name, params, body })
    }


    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = match self.consume(TokenType::Identifier, "ExpectedVariableName") {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let initializer = if self.match_token(vec![&TokenType::Equal]) {
            let expr = match self.expression() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            Some(expr)
        } else {
            None
        };

        if let Err(e) = self.consume(TokenType::Semicolon, "ExpectedSemicolonAfterVariableDeclaration") {
            return Err(e);
        }

        Ok(Stmt::Var{ name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(vec![&TokenType::For]) { return self.for_statement() };
        if self.match_token(vec![&TokenType::If]) { return self.if_statement() };
        if self.match_token(vec![&TokenType::Print]) { return self.print_statement() };
        if self.match_token(vec![&TokenType::Return]) { return self.return_statement() };
        if self.match_token(vec![&TokenType::While]) { return self.while_statement() };
        if self.match_token(vec![&TokenType::LBrace]) { return Ok(Stmt::Block { statements: self.block()? })};

        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        if let Err(e) = self.consume(TokenType::LParen, "ExpectedLParenAfterFor") {
            return Err(e);
        }

        let initializer;
        if self.match_token(vec![&TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(vec![&TokenType::Var]) {
            let var_declaration = match self.var_declaration() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            initializer = Some(Box::new(var_declaration));
        } else {
            let expr_stmt = match self.expression_statement() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            initializer = Some(Box::new(expr_stmt));
        }

        let condition;
        if !self.check(TokenType::Semicolon) {
            condition = match self.expression() {
                Ok(v) => v,
                Err(e) => return Err(e)
            }
        } else {
            condition = Expr::Literal{ value: LiteralType::True };
        }

        if let Err(e) = self.consume(TokenType::Semicolon, "ExpectedSemiColonAfterForCondition") {
            return Err(e)
        }

        let mut increment = None;
        if !self.check(TokenType::RParen) {
            let incr = match self.expression() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            increment = Some(incr);
        }

        if let Err(e) = self.consume(TokenType::RParen, "ExpectedRParenAfterForClauses") {
            return Err(e)
        }

        let body = match self.statement() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        Ok(Stmt::For{ initializer, condition, increment, body: Box::new(body) })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        if let Err(e) = self.consume(TokenType::LParen, "ExpectedLParenAfterIf") {
            return Err(e);
        };
        let condition = match self.expression() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        if let Err(e) = self.consume(TokenType::RParen, "ExpectedLParenAfterCondition") {
            return Err(e);
        };

        let then_branch = match self.statement() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        let mut else_branch = None;
        if self.match_token(vec![&TokenType::Else]) {
            let result = match self.statement() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            else_branch = Some(Box::new(result));
        };
        Ok(Stmt::If{ condition, then_branch: Box::new(then_branch), else_branch })
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = match self.expression() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        if let Err(e) = self.consume(TokenType::Semicolon, "ExpectedSemicolonAfterPrintValue") {
            return Err(e)
        };
        Ok(Stmt::Print{ expression: value })
    }

    fn return_statement(&mut self) -> Result<Stmt, ParserError> {
        let keyword = self.previous().clone();
        let mut value = None;
        if !self.check(TokenType::Semicolon) {
            let expr = match self.expression() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            value = Some(expr);
        }
        if let Err(e) = self.consume(TokenType::Semicolon, "ExpectedSemicolonAfterReturnValue") {
            return Err(e);
        }
        Ok(Stmt::Return{ keyword, value })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        if let Err(e) = self.consume(TokenType::LParen, "ExpectedLParenAfterWhile") {
            return Err(e);
        }
        let condition = match self.expression() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        if let Err(e) = self.consume(TokenType::RParen, "ExpectedLParenAfterCondition") {
            return Err(e);
        }
        let body = match self.statement() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        Ok(Stmt::While{ condition, body: Box::new(body) })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let stmt = match self.declaration() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            statements.push(stmt);
        }

        if let Err(e) = self.consume(TokenType::RBrace, "ExpectedRBraceAfterBlock") {
            return Err(e);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = match self.or() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        if self.match_token(vec![&TokenType::Incr, &TokenType::Decr]) {
            match expr {
                Expr::Var { name } => {
                    match self.previous().token_type {
                        TokenType::Incr => return Ok(Expr::Alteration { name, alteration_type: TokenType::Incr }),
                        TokenType::Decr => return Ok(Expr::Alteration { name, alteration_type: TokenType::Decr }),
                        _ => {
                            let token = self.previous();
                            return Err(
                                ParserError::ExpectedAlterationExpression {
                                start: token.start,
                                end: token.end,
                                line: token.start
                            }
                        )
                        }
                    }
                },
                _ => {
                    let token = self.peek();
                    return Err(
                        ParserError::InvalidAlterationTarget {
                            start: token.start,
                            end: token.end,
                            line: token.line
                        }
                    )
                }
            }
        }

        if self.match_token(vec![&TokenType::Equal]) {
            let value = match self.assignment() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };

            match expr {
                Expr::Var { name } => return Ok(Expr::Assign{ name, value: Box::new(value) }),
                _ => {
                    let token = self.peek();
                    return Err(
                        ParserError::InvalidAssignmentTarget {
                            start: token.start,
                            end: token.end,
                            line: token.line
                        }
                    )
                }
            }
        }

        Ok(expr)
    }
    
    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = match self.and() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        while self.match_token(vec![&TokenType::Or]) {
            let operator = self.previous().clone();
            let right = match self.and() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            expr = Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = match self.equality() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        while self.match_token(vec![&TokenType::And]) {
            let operator = self.previous().clone();
            let right = match self.equality() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            expr = Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = match self.comparison() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        while self.match_token(vec![&TokenType::Bang, &TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = match self.comparison() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary{
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right)
            };
        };
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = match self.term() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        while self.match_token(vec![
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
            &TokenType::BangEqual,
            &TokenType::EqualEqual
            ]) {
                let operator = self.previous().clone();
                let right = match self.term() {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
                expr = Expr::Binary {
                    left: Box::new(expr.clone()),
                    operator,
                    right: Box::new(right)
                };
            }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr =  match self.factor() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        while self.match_token(vec![&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = match self.factor() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = match self.unary() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        while self.match_token(vec![&TokenType::FSlash, &TokenType::Asterisk]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right)
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(vec![&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right)
            })
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = match self.primary() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        
        loop {
            if self.match_token(vec![&TokenType::LParen]) {
                expr = match self.finish_call(expr) {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParserError> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParserError::TooManyArguments{ callee })
                }
                let expr = match self.expression() {
                    Ok(v) => v,
                    Err(e) => return Err(e)
                };
                arguments.push(expr);
                if !self.match_token(vec![&TokenType::Comma]) { break };
            }
        }

        let paren = match self.consume(TokenType::RParen, "ExpectedRParenAfterArguments") {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        Ok(Expr::Call { callee: Box::new(callee), paren, arguments })
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(vec![&TokenType::True]) {
            return Ok(Expr::Literal{ value: LiteralType::True });
        };
        if self.match_token(vec![&TokenType::False]) {
            return Ok(Expr::Literal{ value: LiteralType::False });
        };
        if self.match_token(vec![&TokenType::Nil]) {
            return Ok(Expr::Literal{ value: LiteralType::Nil });
        };
    
        if self.match_token(vec![&TokenType::Num, &TokenType::String]) {
            match self.previous().token_type {
                TokenType::String => return Ok(Expr::Literal { value: LiteralType::Str(self.previous().literal.clone()) }),
                TokenType::Num => {
                    let n = match self.previous().literal.clone().trim().parse() {
                        Ok(v) => v,
                        Err(_) => {
                            let token = self.previous();
                            return Err(
                                ParserError::UnableToParseLiteralToFloat {
                                    start: token.start,
                                    end: token.end,
                                    line: token.line
                                }
                            )
                        }
                    };
                    return Ok(Expr::Literal { value: LiteralType::Num(n) });
                },
                _ => {
                    let token = self.previous();
                    return Err(
                        ParserError::ExpectedStringOrNumber {
                            start: token.start,
                            end: token.end,
                            line: token.line
                        }
                    )
                }
            }
        }

        if self.match_token(vec![&TokenType::Identifier]) {
            let name = self.previous();
            return Ok(Expr::Var { name: name.clone() });
        }

        if self.match_token(vec![&TokenType::LParen]) {
            let expr = match self.expression() {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            if let Err(e) = self.consume(TokenType::RParen, "ExpectedRParenAfterExpression") {
                return Err(e)
            }
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        let token = self.peek();
        Err(ParserError::ExpectedExpression{ start: token.start, end: token.end, line: token.line })
    }
    
    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = match self.expression() {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        if let Err(e) = self.consume(TokenType::Semicolon, "ExpectedExpression") {
            return Err(e)
        }
        Ok(Stmt::Expression{ expression: expr })
    }

    fn match_token(&mut self, types: Vec<&TokenType>) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {return false};
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {self.current += 1};
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {return};

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, error: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        };

        match error {
            "ExpectedVariableName" => {
                let token = self.peek().clone();
                return Err(
                    ParserError::ExpectedVariableName{
                    token_type: token.token_type,
                    lexeme: token.lexeme,
                    line: token.line
                    }
                );
            },
            "ExpectedSemicolonAfterVariableDeclaration" => {
                let token = self.peek().clone();
                return Err(
                    ParserError::ExpectedSemicolonAfterVariableDeclaration {
                        token_type: token.token_type,
                        lexeme: token.lexeme,
                        line: token.line
                    }
                )
            },
            "ExpectedLParenAfterFor" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedLParenAfterFor {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedSemiColonAfterForCondition" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedSemiColonAfterForCondition {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedRParenAfterForClauses" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedRParenAfterForClauses {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedLParenAfterIf" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedLParenAfterIf {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedLParenAfterCondition" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedLParenAfterCondition {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedSemicolonAfterPrintValue" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedSemicolonAfterPrintValue {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedSemicolonAfterReturnValue" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedSemicolonAfterReturnValue {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedLParenAfterWhile" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedLParenAfterWhile {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedRBraceAfterBlock" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedRBraceAfterBlock {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedRParenAfterArguments" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedRParenAfterArguments {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedRParenAfterExpression" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedRParenAfterExpression {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedExpression" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedExpression {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedFunctionName" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedFunctionName {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedLParenAfterFunctionName" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedLParenAfterFunctionName {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            "ExpectedParameterName" => {
                let token = self.peek();
                return Err(
                    ParserError::ExpectedParameterName {
                        start: token.start,
                        end: token.end,
                        line: token.line
                    }
                )
            },
            _ => return Err(ParserError::Unknown)
        }
    }
}
