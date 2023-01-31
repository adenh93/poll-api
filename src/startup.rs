use crate::{
    config::{DatabaseSettings, Settings},
    routes::{create_poll, get_poll, get_poll_results, health_check, vote_poll},
};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(config: Settings, connection_pool: PgPool) -> std::io::Result<Self> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, config.application.base_url)?;

        Ok(Self { server, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> std::io::Result<()> {
        self.server.await
    }
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    let timeout = config.timeout();

    PgPoolOptions::new()
        .acquire_timeout(timeout)
        .connect_lazy_with(config.with_db())
}

fn run(
    listener: TcpListener,
    connection_pool: PgPool,
    base_url: String,
) -> std::io::Result<Server> {
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let connection_pool = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(get_poll)
            .service(vote_poll)
            .service(create_poll)
            .service(get_poll_results)
            .app_data(base_url.clone())
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
