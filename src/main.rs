#![allow(clippy::unused_async)]
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;

async fn teapot() -> impl Responder {
    HttpResponse::InternalServerError()
        .customize()
        .with_status(actix_web::http::StatusCode::IM_A_TEAPOT)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let ip = env::var("IP_ADDRESS").expect("Ip adress should be set");
    let port = env::var("PORT")
        .expect("Port must be set")
        .parse()
        .expect("Invalid port number");

    println!("Running server on {ip}:{port}");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .default_service(web::get().to(teapot))
            .wrap(middleware::Logger::default())
    })
    .bind((ip, port))?
    .run()
    .await
}
