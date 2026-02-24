use crate::{
    evaluator::Evaluator,
    lexer::Lexer,
    parser::Parser,
    semanticanalyser::SemanticAnalyser,
};
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub tokens: String,
    pub ast: String,
    pub output: Vec<String>,
}

pub fn run(source: &str, debug: bool) -> Vec<String> {
    if debug {
        println!("{:?}", source.chars().collect::<Vec<char>>());
    }

    let mut lexer = Lexer::new(source.to_string(), 2);
    let tokens = match lexer.run() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("A lexer error occured: {e}");
            return vec![format!("{e}")];
        }
    };

    if debug {
        println!("Tokens:");
        for token in &tokens {
            println!("{token}");
        }
    }

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("A parser error occured: {e}");
            return vec![format!("{e}")];
        }
    };

    if debug {
        println!("AST:");
        println!("{ast:#?}");
    }

    let mut semantic_analyser = SemanticAnalyser::new(ast.clone());
    match semantic_analyser.run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("A semantic error occured: {e}");
            return vec![format!("{e}")];
        }
    }

    let mut interpreter = Evaluator::new();
    match interpreter.interpret(ast) {
        Ok(output) => {
            return output;
        }
        Err(e) => {
            eprintln!("An interpreter error occured: {e}");
            return vec![format!("{e}")];
        }
    }
}

pub fn run_with_all_outputs(source: &str) -> Output {
    let mut lexer = Lexer::new(source.to_string(), 4);
    let tokens = match lexer.run() {
        Ok(tokens) => tokens,
        Err(e) => {
            return Output {
                tokens: String::new(),
                ast: format!("Lexer error: {e}"),
                output: vec![format!("{e}")],
            };
        }
    };

    let tokens_str = tokens
        .iter()
        .map(|t| format!("{}", t))
        .collect::<Vec<_>>()
        .join("\n");

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            return Output {
                tokens: tokens_str,
                ast: format!("Parser error: {e}"),
                output: vec![format!("{e}")],
            };
        }
    };

    let ast_str = format!("{ast:#?}");

    let mut semantic_analyser = SemanticAnalyser::new(ast.clone());
    if let Err(e) = semantic_analyser.run() {
        return Output {
            tokens: tokens_str,
            ast: ast_str,
            output: vec![format!("Semantic error: {e}")],
        };
    }

    let mut interpreter = Evaluator::new();
    let output = match interpreter.interpret(ast) {
        Ok(output) => output,
        Err(e) => vec![format!("Interpreter error: {e}")],
    };

    Output {
        tokens: tokens_str,
        ast: ast_str,
        output,
    }
}
