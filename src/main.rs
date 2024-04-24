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

#[path = "./semanticanalyser/semanticanalyser.rs"]
mod semanticanalyser;

#[path = "./values/callable.rs"]
mod callable;

#[path = "./values/expr.rs"]
mod expr;

#[path = "./values/list.rs"]
mod list;

#[path = "./values/stmt.rs"]
mod stmt;

#[path = "./values/token.rs"]
mod token;

#[path = "./values/value.rs"]
mod value;

#[cfg(test)]
mod tests;

use rocket::{get, launch, routes};
use rocket::serde::{Deserialize, Serialize, json::Json};
use std::io::Write;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use semanticanalyser::SemanticAnalyser;


#[derive(Serialize, Deserialize)]
struct Message<'r> {
    source: &'r str,
}


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

fn run(source: String) -> Vec<String> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens = match lexer.run() {
        Ok(tokens) => tokens,
        Err(_) => {
            return vec!["error".to_string()];
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("A parser error occured: {e}");
            return vec!["error".to_string()];
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
        Ok(output) => return output,
        Err(e) => {
            eprintln!("An interpreter error occured: {e}")
        }
    }

    return vec!["error".to_string()];
}

#[get("/test", format = "json", data = "<message>")]
fn test(message: Json<Message<'_>>) -> Json<String> {
    let output = run(message.source.to_string());

    Json(format!("{:?}", output))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1", routes![test])
}


// fn main() {
//     let source = repl();

//     let mut lexer = Lexer::new(source);
//     let tokens = match lexer.run() {
//         Ok(tokens) => tokens,
//         Err(e) => {
//             eprintln!("A lexer error occured: {e}");
//             return;
//         }
//     };

//     let mut parser = Parser::new(tokens);
//     let ast = match parser.parse() {
//         Ok(ast) => ast,
//         Err(e) => {
//             eprintln!("A parser error occured: {e}");
//             return;
//         }
//     };

//     let mut semantic_analyser = SemanticAnalyser::new(ast.clone());
//     match semantic_analyser.run() {
//         Ok(_) => {}
//         Err(e) => {
//             eprintln!("A semantic error occured: {e}");
//             return;
//         }
//     }

//     let mut interpreter = Interpreter::new();
//     match interpreter.interpret(ast) {
//         Ok(_) => {}
//         Err(e) => {
//             eprintln!("An interpreter error occured: {e}")
//         }
//     }
// }
