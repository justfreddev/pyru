use interpreter_v1::tokens::{Token, TokenType};

use crate::interpreter;
use crate::expr::{Expr, LiteralType};
use crate::stmt::Stmt;

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
        self.equality()
    }

    fn declaration(&mut self) -> Stmt {
        match self.try_declaration() {
            Ok(stmt) => stmt,
            Err(_) => {
                self.synchronize();
                self.declaration()
            }
        }
    }

    fn try_declaration(&mut self) -> Result<Stmt, ()> {
        if self.match_token(vec![&TokenType::Var]) {
            Ok(self.var_declaration())
        } else {
            Ok(self.statement())
        }
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token(vec![&TokenType::Print]) {return self.print_statement()};

        self.expression_statement()
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

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Stmt::Expression{ expression: expr }
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
        self.primary()
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

        println!("{}", self.peek().clone());

        interpreter::Interpreter::token_error(self.peek(), "Expected expression.");
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

        interpreter::Interpreter::token_error(self.peek(), message);
        panic!("Parse error.");
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
}