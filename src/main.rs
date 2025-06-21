use anyhow::Result;
use axum::{Router, routing::get};
use newsletter::{configuration, telemetry};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = configuration::Settings::load()?;
    let _guards = telemetry::init_tracing(configuration.log_settings);

    let app = Router::new().route("/", get(|| async { "Hello, world" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
