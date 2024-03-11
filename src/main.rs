#[path ="./lexer/lexer.rs"]
mod lexer;
use std::io::Write;
use lexer::Lexer;

struct Interpreter {}

impl Interpreter {
    fn new() -> Self {
        Self {}
    }

    fn run(&self, source: String) {
        let mut lexer = Lexer::new(source);
        lexer.scan();

        for token in lexer.tokens {
            println!("{token}");
        }
    }
}

fn main() {
    let interpreter = Interpreter::new();

    loop {
        let mut inp = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut inp).unwrap();

        interpreter.run(inp);
    }
}