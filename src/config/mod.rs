pub mod crypto;

use std::sync::Arc;

use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenvy::dotenv;
use r2d2::{Pool, PooledConnection};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;
use eyre::WrapErr;
use color_eyre::Result;
use serde::Deserialize;

use crypto::CryptoService;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
// pub type PostgresPool = PooledConnection<ConnectionManager<PgConnection>>;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
            .context("loading configuration from environment")
    }

    #[instrument(skip(self))]
    pub fn db_pool(&self) -> PostgresPool {
        info!("Creating database connection pool.");
        let manager = ConnectionManager::<PgConnection>::new(&self.database_url);
        // let pool = Pool::new(manager).expect("Failed to create pool.");
        // let conn = pool.get().expect("Failed to get connection from pool.");
        // conn
        r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(manager)
        .expect("could not build connection pool")
    }

    #[instrument(skip(self))]
    pub fn hashing(&self) -> CryptoService {
        CryptoService {
            key_password: Arc::new(self.secret_key.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }
}
