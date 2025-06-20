use anyhow::Result;
use newsletter::{configuration, telemetry};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = configuration::Settings::load()?;
    println!("{:?}", serde_json::to_string(&configuration).unwrap());
    // let _guards = telemetry::init_tracing(configuration.log_settings);
    Ok(())
}
