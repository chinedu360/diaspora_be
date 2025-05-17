use serde::Deserialize;
use config::Config;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSetting,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSetting {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database_name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // init our configuration reader
    let settings = Config::builder();

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.

    // Layer 1: Default configuration
    let settings = settings.add_source(config::File::with_name("configuration/base"));

    // Try to convert the configuration values it read into
    // our Settings type
    settings.build()?.try_deserialize()
}