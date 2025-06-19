use anyhow::Result;
use newsletter::{configuration, telemetry};

#[tokio::main]
async fn main() -> Result<()> {
    let _configuration = configuration::Settings::load()?;
    let _guards = telemetry::tracing_init()?;
    todo!()
}
