use interpreter_v1::tokens::{Token, TokenType};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::expr::AstPrinter;

pub struct Interpreter {
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, source: String) {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        println!("{}", AstPrinter.print(&expression));
    }

    pub fn line_error(line: usize, message: &str) {
        Interpreter::report(line, "", message);
    }
    
    pub fn report(line: usize, where_about: &str, message: &str) {
        println!("[line {line}] Error {where_about}: {message}");
    }

    pub fn token_error(token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            Interpreter::report(token.line, " at end", message);
        } else {
            Interpreter::report(token.line, format!(" at '{}'", token.lexeme).as_str(), message);
        }
    }
}
