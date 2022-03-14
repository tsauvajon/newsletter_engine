use config;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub db: Database,
}

#[derive(serde::Deserialize, Debug)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::new("config", config::FileFormat::Yaml))
        // .add_source(config::Environment::with_prefix("Z2P"))
        .build()?
        .try_deserialize::<Config>()
}
