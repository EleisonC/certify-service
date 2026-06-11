use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info,certify-service-backend=debug"))?;

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().compact())
        .with(ErrorLayer::default())
        .try_init()?;

    Ok(())
}
