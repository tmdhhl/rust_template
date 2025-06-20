use anyhow::Result;
use newsletter::{configuration, telemetry};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = configuration::Settings::load()?;
    let _guards = telemetry::init_tracing(configuration.log_settings);

    tracing::error!("nihao");
    Ok(())
}
