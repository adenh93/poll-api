use crate::routes::health_check;
use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(move || App::new().service(health_check))
        .listen(listener)?
        .run();

    Ok(server)
}
