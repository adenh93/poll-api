use poll_api::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:4000")?;
    let server = run(listener)?;

    server.await?;

    Ok(())
}
