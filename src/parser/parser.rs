use crate::{
    expr::{Expr, LiteralType},
    interpreter::Interpreter,
    stmt::Stmt,
    tokens::{Token, TokenType}
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }

        statements
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn declaration(&mut self) -> Stmt {
        self.try_declaration()
    }

    fn try_declaration(&mut self) -> Stmt {
        if self.match_token(vec![&TokenType::Fun]) {
            self.function("function".to_string())
        } else if self.match_token(vec![&TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token(vec![&TokenType::For]) { return self.for_statement() };
        if self.match_token(vec![&TokenType::If]) { return self.if_statement() };
        if self.match_token(vec![&TokenType::Print]) { return self.print_statement() };
        if self.match_token(vec![&TokenType::While]) { return self.while_statement() };
        if self.match_token(vec![&TokenType::LeftBrace]) { return Stmt::Block { statements: self.block() }};

        self.expression_statement()
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition.");

        let then_branch = self.statement();
        let mut else_branch = None;
        if self.match_token(vec![&TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()));
        };
        Stmt::If { condition, then_branch: Box::new(then_branch), else_branch: else_branch }
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print{ expression: value }
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect variable name.");

        let initializer = if self.match_token(vec![&TokenType::Equal]) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration");

        Stmt::Var { name, initializer }
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");
        let body = self.statement();

        Stmt::While { condition, body: Box::new(body) }
    }

    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        let initializer;
        if self.match_token(vec![&TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(vec![&TokenType::Var]) {
            initializer = Some(self.var_declaration());
        } else {
            initializer = Some(self.expression_statement());
        }

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression());
        }

        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression());
        }

        self.consume(TokenType::RightParen, "Expect ')' after for clauses.");

        let mut body = self.statement();

        if let Some(incr) = increment {
            body = Stmt::Block { statements: vec![body, Stmt::Expression { expression: incr }] }
        }

        if condition.is_none() {
            condition = Some(Expr::Literal { value: LiteralType::True });
        }
        body = Stmt::While { condition: condition.unwrap(), body: Box::new(body) };

        if let Some(init) = initializer {
            body = Stmt::Block { statements: vec![init, body] };
        }

        body
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Stmt::Expression{ expression: expr }
    }

    fn function(&mut self, kind: String) -> Stmt {
        let name = self.consume(TokenType::Identifier, format!("Expect {kind} name.").as_str());
        self.consume(TokenType::LeftParen, format!("Expect '(' after {kind} name.").as_str());
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    Interpreter::token_error(self.peek(), format!("Can't have more than 255 parameters.").as_str());
                }

                parameters.push(self.consume(TokenType::Identifier, format!("Expect parameter name.").as_str()));
                if !self.match_token(vec![&TokenType::Comma]) { break };
            }
        }
        self.consume(TokenType::RightParen, format!("Expect ')' after parameters.").as_str());
        
        self.consume(TokenType::LeftBrace, format!("Expect '{{' before {kind} body.").as_str());
        let body = self.block();
        Stmt::Function { name, params: parameters, body }
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        statements
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.match_token(vec![&TokenType::Incr, &TokenType::Decr]) {
            match expr {
                Expr::Var { name } => {
                    match self.previous().token_type {
                        TokenType::Incr => return Expr::Alteration { name, alteration_type: TokenType::Incr },
                        TokenType::Decr => return Expr::Alteration { name, alteration_type: TokenType::Decr },
                        _ => panic!("Expected an alteration expression")
                    }
                },
                _ => panic!("Invalid alteration target")
            }
        }

        if self.match_token(vec![&TokenType::Equal]) {
            let value = self.assignment();

            match expr {
                Expr::Var { name } => return Expr::Assign { name, value: Box::new(value) },
                _ => panic!("Invalid assignment target")
            }
        }

        expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();
        while self.match_token(vec![&TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and();
            expr = Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) };
        }

        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.match_token(vec![&TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality();
            expr = Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) }
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_token(vec![&TokenType::Bang, &TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary{
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right)
            };
        };
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_token(vec![
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
            &TokenType::BangEqual,
            &TokenType::EqualEqual
            ]) {
                let operator = self.previous().clone();
                let right = self.term();
                expr = Expr::Binary {
                    left: Box::new(expr.clone()),
                    operator,
                    right: Box::new(right)
                };
            }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(vec![&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right)
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(vec![&TokenType::FSlash, &TokenType::Asterisk]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right)
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right)
            }
        }

        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    Interpreter::token_error(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(self.expression());
                if !self.match_token(vec![&TokenType::Comma]) { break };
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments");

        Expr::Call { callee: Box::new(callee), paren, arguments }
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        
        loop {
            if self.match_token(vec![&TokenType::LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }

        expr
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![&TokenType::True]) {
            return Expr::Literal{ value: LiteralType::True };
        };
        if self.match_token(vec![&TokenType::False]) {
            return Expr::Literal{ value: LiteralType::False };
        };
        if self.match_token(vec![&TokenType::Nil]) {
            return Expr::Literal{ value: LiteralType::Nil };
        };
    
        if self.match_token(vec![&TokenType::Num, &TokenType::String]) {
            match self.previous().token_type {
                TokenType::String => return Expr::Literal { value: LiteralType::Str(self.previous().literal.clone()) },
                TokenType::Num => {
                    let n = self.previous().literal.clone().trim().parse().expect("Why tf");
                    return Expr::Literal { value: LiteralType::Num(n) };
                },
                _ => panic!("Expected a String or Num TokenType")
            }
        }

        if self.match_token(vec![&TokenType::Identifier]) {
            let name = self.previous();
            return Expr::Var { name: name.clone() };
        }

        if self.match_token(vec![&TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping { expression: Box::new(expr) };
        }

        Interpreter::token_error(self.peek(), "Expected expression.");
        Expr::Literal{ value: LiteralType::Nil }
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

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance().clone();
        };

        Interpreter::token_error(self.peek(), message);
        panic!("Parse error.");
    }

    fn _synchronize(&mut self) {
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
}