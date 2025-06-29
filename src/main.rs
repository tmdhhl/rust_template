use anyhow::Result;
use kuai_saver::{configuration, startup::Application, telemetry};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = configuration::Settings::load()?;
    let (subscriber, _guards) = telemetry::init_tracing(configuration.log_settings.clone());
    telemetry::set_subscriber(subscriber);

    let app = Application::build(configuration).await?;
    app.run_until_stopped().await?;

    Ok(())
}
