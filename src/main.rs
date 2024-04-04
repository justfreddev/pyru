#[path = "./interpreter/interpreter.rs"]
mod interpreter;

#[path = "./lexer/lexer.rs"]
mod lexer;

#[path = "./lexer/tokens.rs"]
mod tokens;

#[path = "./parser/environment.rs"]
mod environment;

#[path = "./parser/expr.rs"]
mod expr;

#[path = "./parser/parser.rs"]
mod parser;

#[path = "./parser/stmt.rs"]
mod stmt;

#[path = "./macros.rs"]
mod macros;

use std::io::Write;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

// Runs the interpreter
fn run(source: String) {
    let mut lexer = Lexer::new(source); // Initialises lexer with the source code
    let tokens = lexer.scan(); // Lexer scans the raw input and generates a vector of tokens from them

    let mut parser = Parser::new(tokens); // Initialises parser with the vector of tokens
    let statements = parser.parse(); // Parser generates a vector of statements from the tokens

    // for stmt in &statements {
    //     println!("{stmt}");
    // }

    let mut interpreter = Interpreter::new(); // Initalises interpreter
    interpreter.interpret(statements); // Interpret the vector of statements and generate an output
}

// Creates a REPL for the end-user to enter their source code and then returns it
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
    let source = repl(); // Get the source code
    run(source); // Run the interpreter with the source code
}
