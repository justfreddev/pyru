#[path = "./interpreter/interpreter.rs"]
mod interpreter;

#[path = "./lexer/lexer.rs"]
mod lexer;

#[path = "./lexer/tokens.rs"]
mod tokens;

#[path ="./parser/environment.rs"]
mod environment;

#[path = "./parser/expr.rs"]
mod expr;

#[path = "./parser/parser.rs"]
mod parser;

#[path = "./parser/stmt.rs"]
mod stmt;

use std::io::Write;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn run(interpreter: &mut Interpreter, source: String) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    println!("{:#?}", statements);

    interpreter.interpret(statements);
}

fn main() {
    let mut interpreter = Interpreter::new();
    let mut source = String::new();
    loop {
        let mut temp_source = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut temp_source).unwrap();
        if temp_source.trim().eq("run") || temp_source.trim().eq("") {
            run(&mut interpreter, source.clone());
            source.clear();
        } else {
            source.push_str(&temp_source);
        }
    }
    
}