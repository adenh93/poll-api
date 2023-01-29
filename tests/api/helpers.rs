use poll_api::{
    config::{get_config, DatabaseSettings},
    startup::Application,
};
use reqwest::Response;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub client: reqwest::Client,
    pub connection_pool: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        let mut config = get_config().expect("Failed to load config");
        config.application.port = 0;
        config.database.database_name = Uuid::new_v4().to_string();

        let connection_pool = configure_database(&config.database).await;

        let application = Application::build(config, connection_pool.clone())
            .await
            .expect("Failed to build application");

        let application_port = application.port();
        let address = format!("http://127.0.0.1:{}", application_port);
        let _ = tokio::spawn(application.run_until_stopped());
        let client = reqwest::Client::new();

        Self {
            address,
            port: application_port,
            client,
            connection_pool,
        }
    }

    pub async fn get_poll(&self, uuid: &Uuid) -> Response {
        self.client
            .get(&format!("{}/polls/{}", &self.address, &uuid.to_string()))
            .send()
            .await
            .expect("Failed to execute request")
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create test database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
