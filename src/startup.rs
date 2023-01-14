use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::io::Result;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
