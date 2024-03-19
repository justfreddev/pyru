#[path = "./parser/expr.rs"]
mod expr;

#[path = "./parser/stmt.rs"]
mod stmt;

#[path = "./lexer/lexer.rs"]
mod lexer;

#[path = "./parser/parser.rs"]
mod parser;

#[path ="./parser/environment.rs"]
mod environment;

#[path = "./interpreter/interpreter.rs"]
mod interpreter;

use std::io::Write;
use crate::lexer::Lexer;
use crate::parser::Parser;
// use crate::expr::AstPrinter;

use interpreter::Interpreter;

fn run(interpreter: &mut Interpreter, source: String) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    // println!("{}", AstPrinter.print(&expression));

    interpreter.interpret(statements);
}

fn main() {
    let mut interpreter = Interpreter::new();
    loop {
        let mut source = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut source).unwrap();
        
        run(&mut interpreter, source);
    }
    
}