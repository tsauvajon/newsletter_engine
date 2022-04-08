use config;
use sqlx::{Connection, Executor, PgConnection, PgPool};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub app: Application,
    pub db: Database,
}

#[derive(serde::Deserialize, Debug)]
pub struct Application {
    pub port: u16,
    pub host: String,
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
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Unknown environment"); // TODO: return an Err instead

    let config_path = configuration_directory.join(env.as_str());

    config::Config::builder()
        .add_source(config::File::from(config_path))
        .build()?
        .try_deserialize::<Settings>()
}

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
            unknown => Err(format!("unknown environment '{}'", unknown)),
        }
    }
}
