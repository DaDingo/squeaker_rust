use actix_settings::BasicSettings;
use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Opts {
    pub database_url: String,
    pub health_body: String,
}

pub trait ReadConfig {
    fn read_config() -> Result<BasicSettings<Opts>, ConfigError>;
}

impl ReadConfig for BasicSettings<Opts> {
    fn read_config() -> Result<BasicSettings<Opts>, ConfigError> {
        Config::builder()
            .add_source(File::new("Server.toml", FileFormat::Toml))
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
