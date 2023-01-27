use poll_api::{
    config::get_config,
    startup::{get_connection_pool, Application},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read config");
    let connection_pool = get_connection_pool(&config.database);
    let application = Application::build(config, connection_pool).await?;

    application.run_until_stopped().await?;

    Ok(())
}
