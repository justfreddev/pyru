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

use actix_cors::Cors;
use actix_web::{ web, App, HttpServer, Responder };
use serde::{ Deserialize, Serialize };

use crate::run::run_with_all_outputs;

#[derive(Serialize, Deserialize)]
struct Message {
    source: String,
}

async fn run_code(message: web::Json<Message>) -> impl Responder {
    let output = run_with_all_outputs(message.source.as_str());
    web::Json(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env
          ::var("PORT")
          .unwrap_or_else(|_| "8080".to_string())
          .parse()
          .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(web::scope("/v1").route("/runcode", web::post().to(run_code)))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

// fn _main() {
//     let source = _repl();

//     let debug = false;

//     let _ = run(source.as_str(), debug);
// }
