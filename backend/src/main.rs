//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_web::http::{header};
use actix_web::{middleware::Logger, App, HttpServer };

mod entity;
mod usecase;
mod rest;
mod database;
mod schema;
mod gateway;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let host = std::env::var("BIND_HOST").expect(".envファイルからBIND_HOSTを取得できませんでした。");
    let port = std::env::var("BIND_PORT").expect(".envファイルからBIND_PORTを取得できませんでした。");
        
    let bind = format!("{}:{}", host, port);

    println!("Starting server at: {}", &bind);
    // Start HTTP server
    let server = HttpServer::new(move || {
        let cors_url = std::env::var("CORS_URL").expect(".envファイルからCORS_URLを取得できませんでした。");
        let cors = Cors::new()
            .allowed_origin(&cors_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .finish();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(rest::get_user)
            .service(rest::add_user)
            .service(rest::ping)
    }).bind(&bind)?;

    server.run().await
}