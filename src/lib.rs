use anyhow::Result;
use axum::Router;
use tokio::{net::TcpListener, task::JoinHandle};

mod api;

/// Run
pub async fn run(listener: TcpListener) -> Result<()> {
    let app = Router::new()
        .nest("/v2", api::v2::routing());
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

/// Spawn a test service as task and return the join handle
pub fn spawn(listener: TcpListener) -> Result<JoinHandle<Result<()>>> {
    let fut = run(listener);
    let jh = tokio::spawn(fut);
    Ok(jh)
}
