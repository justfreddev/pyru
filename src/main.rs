mod error;

#[path = "./evaluator/environment.rs"]
mod environment;

#[path = "./evaluator/evaluator.rs"]
mod evaluator;

#[path = "./lexer/lexer.rs"]
mod lexer;

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

mod run;

#[path = "./values/stmt.rs"]
mod stmt;

#[path = "./values/token.rs"]
mod token;

#[path = "./values/value.rs"]
mod value;

#[cfg(test)]
mod tests;

#[allow(unused)]
use rocket::{http::Method, launch, post, routes};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::io::Write;

use run::run;

#[derive(Serialize, Deserialize)]
struct Message {
    source: String,
}

fn _repl() -> String {
    let mut source = String::new();
    loop {
        let mut temp_source = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut temp_source).unwrap();
        if temp_source.trim().eq("run") || temp_source.trim().eq("") {
            // LIST OPERATIONS - BAND A
            return source
                .chars()
                .collect::<Vec<char>>()[0..source.len()-3]
                .iter()
                .collect::<String>();
        }
        temp_source.push('\n');
        source.push_str(&temp_source);
    }
}


fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
        "http://localhost:5173"
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

// COMPLEX CLIENT-SERVER MODEL - BAND A
#[post("/runcode", format = "json", data = "<message>")]
fn run_code(message: Json<Message>) -> Json<String> {
    let debug = false;
    let output = run(message.source.as_str(), debug);

    // JSON PARSING - BAND A
    Json(format!("{:?}", output))
}

// COMPLEX CLIENT-SERVER MODEL - BAND A
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1", routes![run_code]).attach(make_cors())
}


fn _main() {
    let source = _repl();

    let debug = false;

    let _ = run(source.as_str(), debug);
}
