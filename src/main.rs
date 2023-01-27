use poll_api::{config::get_config, startup::Application};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read config");
    let application = Application::build(config).await?;

    application.run_until_stopped().await?;

    Ok(())
}
