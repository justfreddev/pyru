#[path = "./parser/expr.rs"]
mod expr;

#[path = "./lexer/lexer.rs"]
mod lexer;

#[path = "./parser/parser.rs"]
mod parser;

#[path = "./interpreter/interpreter.rs"]
mod interpreter;

use std::io::Write;

use interpreter::Interpreter;


fn main() {
    let mut source = String::new();
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut source).unwrap();
    
    let interpreter = Interpreter::new();
    interpreter.run(source);
}