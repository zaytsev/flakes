use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

pub async fn start(host: &str, port: u16, router: Router) -> Result<(), std::io::Error> {
    info!("Starting web server {}:{}", host, port);
    let bind_address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(bind_address).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
