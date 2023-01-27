use crate::{config::Settings, routes::health_check};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(config: Settings) -> std::io::Result<Self> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, config.application.base_url)?;

        Ok(Self { server, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> std::io::Result<()> {
        self.server.await
    }
}

fn run(listener: TcpListener, base_url: String) -> std::io::Result<Server> {
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));

    let server =
        HttpServer::new(move || App::new().service(health_check).app_data(base_url.clone()))
            .listen(listener)?
            .run();

    Ok(server)
}
