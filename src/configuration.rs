use config::Config;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSetting,
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSetting {
    pub fn connection_string(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name
            )
            .into(),
        )
    }
}

/// App runtime env
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Step 1: Get the current directory to build paths
    let base_path = std::env::current_dir().expect("Failed to dtermine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Step 2: Start building the configuration
    let settings = Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")).required(true));

    // Step 3: Detect which environment we're running in
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Step 4: Layer on environment-specific configuration
    let settings = settings.add_source(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    );

    // Step 5: Build the final config and deserialize into our Settings struct
    settings.build()?.try_deserialize()
}
