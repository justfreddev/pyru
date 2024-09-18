mod error;

#[path = "./interpreter/environment.rs"]
mod enviromnent;

#[path = "./interpreter/interpreter.rs"]
mod interpreter;

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

#[path = "./tests/interpreter_tests.rs"]
mod interpreter_tests;

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

fn repl() -> String {
    let mut source = String::new();
    loop {
        let mut temp_source = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut temp_source).unwrap();
        if temp_source.trim().eq("run") || temp_source.trim().eq("") {
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


fn _make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
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


#[post("/runcode", format = "json", data = "<message>")]
fn _run_code(message: Json<Message>) -> Json<String> {
    let debug = false;
    let output = run(message.source.as_str(), debug);

    Json(format!("{:?}", output))
}


// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/v1", routes![run_code]).attach(make_cors())
// }


fn main() {
    let source = repl();

    let debug = true;

    let _ = run(source.as_str(), debug);
}
