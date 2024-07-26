use anyhow::Result;
use axum::Router;

mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
    let jh = test_oci_registry::spawn(listener)?;
    jh.await??;
    Ok(())
}
