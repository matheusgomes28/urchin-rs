use std::{net::Ipv4Addr, time::Duration};

use sea_orm::{ActiveModelTrait, ConnectOptions};

mod posts;
mod app_error;

pub use app_error::AppError;

pub struct Database {
    /// The IP for the database connection
    pub ip: Ipv4Addr,
    /// Port for the database connection
    pub port: u16,

    /// The underlying db connection
    _db_connection: sea_orm::DatabaseConnection,
}

impl Database {
    // TODO : this has to take the database name, protocol, user, pass, etc
    pub async fn new(ip: &str, port: u16) -> anyhow::Result<Self> {
        // build the connection string here
        let conn_str = format!("mysql://root:root@{ip}/urchin_rs");

        // TODO : make this configurable from a file
        let mut opt = ConnectOptions::new(conn_str);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

        let db = sea_orm::Database::connect(opt).await?;

        // Parse the string to IP
        let ip = ip.parse::<Ipv4Addr>()?;

        Ok(Database {
            ip,
            port,
            _db_connection: db,
        })
    }

    pub async fn add_post(&self, title: &str, excerpt: &str, content: &str) -> anyhow::Result<i32> {
        // insert everything into db with ORM
        let post = posts::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            title: sea_orm::ActiveValue::Set(title.to_string()),
            content: sea_orm::ActiveValue::Set(content.to_string()),
            excerpt: sea_orm::ActiveValue::Set(excerpt.to_string()),
        };

        let ent = post.insert(&self._db_connection)
            .await
            .map_err(anyhow::Error::msg)?;

        let inserted_id = ent.id;

        Ok(inserted_id)
    }
}
