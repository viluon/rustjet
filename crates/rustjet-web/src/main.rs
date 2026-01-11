mod routes;

use anyhow::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("rustjet-web listening on {}", addr);

    let app = routes::create_router();

    axum::serve(listener, app).await?;

    Ok(())
}
