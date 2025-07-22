use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct TrancerConfig {
    pub server: TrancerServerConfig,
    pub general: TrancerGeneralConfig,
    pub roles: TrancerRolesConfig,
    pub analytics: TrancerAnalyticsConfig,
    pub payouts: TrancerPayoutsConfig,
    pub economy: TrancerEconomyConfig,
}

#[derive(Debug, Deserialize)]
pub struct TrancerGeneralConfig {
    pub data_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct TrancerServerConfig {
    pub id: String,
    pub invite_link: String,
}

#[derive(Debug, Deserialize)]
pub struct TrancerRolesConfig {
    pub birthday: String,
    pub can_request: String,
}

#[derive(Debug, Deserialize)]
pub struct TrancerAnalyticsConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct TrancerEconomyConfig {
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct TrancerPayoutsConfig {
    pub bumps: TrancerBumpPayoutsConfig,
}

#[derive(Debug, Deserialize)]
pub struct TrancerBumpPayoutsConfig {
    pub currency_min: u32,
    pub currency_max: u32,
}

lazy_static! {
    pub static ref CONFIG: TrancerConfig = {
        let config = Config::builder()
            .add_source(config::File::with_name("config_dev").required(false))
            .add_source(config::File::with_name("config").required(false))
            .build()
            .unwrap()
            .try_deserialize::<TrancerConfig>()
            .unwrap();

        config
    };
}

#[instrument]
pub(crate) fn load_config() -> Result<TrancerConfig, config::ConfigError> {
    info!("Loading config");
    let config = Config::builder()
        .add_source(config::File::with_name("config_dev").required(false))
        .add_source(config::File::with_name("config").required(false))
        .build()?
        .try_deserialize::<TrancerConfig>()?;

    Ok(config)
}
