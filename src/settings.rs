use std::net::IpAddr;

use config::{Config, Environment, File};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::RedisPool;

impl Settings {
    pub fn load_settings() -> Self {
        let app_mode = std::env::var("APP_MODE").unwrap_or_else(|_| "local".to_string());

        let config = Config::builder()
            .add_source(File::with_name("config/base.toml"))
            .add_source(File::with_name(&format!("config/{}.toml", app_mode)))
            .add_source(Environment::with_prefix("APP_"))
            .build()
            .expect("should be able to build config data from its data sources");

        config
            .try_deserialize()
            .expect("should be able to deserialize config to settings")
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub http: HttpSettings,
    pub postgres: PostgresSettings,
    pub redis: RedisSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpSettings {
    pub port: u16,
    pub ip: IpAddr,
}

impl HttpSettings {
    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database: String,
    pub max_connections: u32,
}

impl PostgresSettings {
    pub async fn create_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&self.connection_string())
            .await
    }

    fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub password: SecretString,
}

impl RedisSettings {
    pub async fn create_pool(&self) -> anyhow::Result<RedisPool> {
        let url = format!(
            "redis://:{}@{}/{}",
            self.password.expose_secret(),
            self.host,
            self.port
        );
        let config = deadpool_redis::Config::from_url(&url);

        Ok(config.create_pool(Some(deadpool_redis::Runtime::Tokio1))?)
    }
}
