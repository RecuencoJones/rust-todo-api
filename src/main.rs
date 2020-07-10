use actix_web::{middleware, App, HttpServer};
use dotenv;
use env_logger;

mod controllers;
mod models;
mod repository;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let port = dotenv::var("RUST_PORT").unwrap_or(String::from("8080"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath)
            .service(controllers::todo::controller())
    })
    .bind(format!("127.0.0.1:{port}", port = port))?
    .run()
    .await
}
