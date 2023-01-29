use poll_api::{
    config::get_config,
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("poll-api", "info", std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config");
    let connection_pool = get_connection_pool(&config.database);
    let application = Application::build(config, connection_pool).await?;

    application.run_until_stopped().await?;

    Ok(())
}
