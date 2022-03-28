use config;
use sqlx::{Connection, Executor, PgConnection, PgPool};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
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
        format!("{}/{}", self.connection_string_without_db(), self.name)
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }

    pub async fn configure(&self) -> Result<PgPool, sqlx::Error> {
        let mut connection =
            PgConnection::connect(self.connection_string_without_db().as_str()).await?;

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, self.name).as_str())
            .await?;

        let pool = PgPool::connect(self.connection_string().as_str()).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(pool)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::new("config", config::FileFormat::Yaml))
        .build()?
        .try_deserialize::<Settings>()
}
