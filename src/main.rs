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

#[path = "./tests/interpreter_tests.rs"]
mod interpreter_tests;

#[cfg(test)]
mod tests;

use rocket::{launch, post, routes};
use rocket::serde::{Deserialize, Serialize, json::Json};
use std::io::Write;

use interpreter_tests::run;


#[derive(Serialize, Deserialize)]
struct Message<'r> {
    source: &'r str,
}


fn _repl() -> String {
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

#[post("/test", format = "json", data = "<message>")]
fn test(message: Json<Message<'_>>) -> Json<String> {
    let output = run(message.source);

    Json(format!("{:?}", output))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1", routes![test])
}


// fn main() {
//     let source = repl();

//     run(source.as_str());
// }
