#[path ="./lexer/lexer.rs"]
mod lexer;

#[path = "./parser/expr.rs"]
mod expr;

use std::io::Write;
use lexer::Lexer;
use expr::run_ast;

struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            had_error: false,
        }
    }

    fn run(&mut self, source: String) {
        let mut lexer = Lexer::new(source);
        self.had_error = lexer.scan();

        for token in lexer.tokens {
            println!("{token}");
        }
        self.had_error = false;
    }
}

fn main() {

    run_ast();

    // let mut interpreter = Interpreter::new();

    // loop {
    //     let mut inp = String::new();
    //     print!("> ");
    //     std::io::stdout().flush().unwrap();
    //     std::io::stdin().read_line(&mut inp).unwrap();

    //     interpreter.run(inp);
    // }
}