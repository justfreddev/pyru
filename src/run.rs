use crate::{
    interpreter::Interpreter,
    lexer::Lexer,
    parser::Parser,
    semanticanalyser::SemanticAnalyser
};

pub fn run(source: &str) {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = match lexer.run() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("A lexer error occured: {e}");
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("A parser error occured: {e}");
            return;
        }
    };

    let mut semantic_analyser = SemanticAnalyser::new(ast.clone());
    match semantic_analyser.run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("A semantic error occured: {e}");
        }
    }

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(ast) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("An interpreter error occured: {e}");
        }
    }
}