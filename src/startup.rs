use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::io::Result;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
