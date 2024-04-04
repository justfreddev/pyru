#[path = "./error.rs"]
mod error;

#[path = "./interpreter/environment.rs"]
mod enviromnent;

#[path = "./interpreter/interpreter.rs"]
mod interpreter;

#[path = "./lexer/lexer.rs"]
mod lexer;

#[path = "./macros.rs"]
mod macros;

#[path = "./parser/parser.rs"]
mod parser;

#[path = "./values/callable.rs"]
mod callable;

#[path = "./values/expr.rs"]
mod expr;

#[path = "./values/stmt.rs"]
mod stmt;

#[path = "./values/token.rs"]
mod token;

#[path = "./values/value.rs"]
mod value;

use std::io::Write;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn repl() -> String {
    let mut source = String::new();
    loop {
        let mut temp_source = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut temp_source).unwrap();
        if temp_source.trim().eq("run") || temp_source.trim().eq("") {
            return source;
        }
        temp_source.push('\n');
        source.push_str(&temp_source);
    }
}

fn main() {
    let source = repl();

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.run() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("A lexer error occured: {e}");
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => {
            eprintln!("A parser error occured: {e}");
            return;
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(statements) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("An interpreter error occured: {e}")
        }
    }
}
