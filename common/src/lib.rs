use std::{net::Ipv4Addr, time::Duration};

use anyhow::Context;
use http::StatusCode;
use sea_orm::{ActiveModelTrait, ConnectOptions, EntityTrait, ModelTrait};

mod app_error;
mod config;
mod posts;

pub use app_error::AppError;
pub use config::UrchinConfig;
pub use posts::{AddPostRequest, AddPostResponse, DeletePostResponse, GetPostResponse};

// TODO : Move all of the database code elsewhere

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
        let conn_str = format!("mysql://root:root@{ip}:{port}/urchin_rs");

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

        let ent = post
            .insert(&self._db_connection)
            .await
            .map_err(anyhow::Error::msg)?;

        let inserted_id = ent.id;

        Ok(inserted_id)
    }

    pub async fn get_post(&self, post_id: i32) -> anyhow::Result<GetPostResponse, AppError> {
        // insert everything into db with ORM
        let post = posts::Entity::find_by_id(post_id)
            .one(&self._db_connection)
            .await
            .map_err(|e| AppError {
                err_msg: e.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?
            .context("could not find post id in database")
            .map_err(|e| AppError {
                err_msg: e.to_string(),
                status_code: StatusCode::BAD_REQUEST,
            })?;

        Ok(GetPostResponse {
            content: post.content,
            excerpt: post.excerpt,
            post_id: post.id,
            title: post.title,
        })
    }

    pub async fn get_posts(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Vec<GetPostResponse>, AppError> {
        // insert everything into db with ORM
        if offset.is_negative() || limit.is_negative() {
            return Err(AppError {
                err_msg: "page number cannot be negative".into(),
                status_code: StatusCode::BAD_REQUEST,
            });
        }

        let start_offset = offset * limit;
        let end_offset = (offset + 1) * limit;

        let posts = posts::Entity::find()
            .cursor_by(posts::Column::Id)
            .after(start_offset)
            .before(end_offset)
            .all(&self._db_connection)
            .await
            .map_err(|e| AppError {
                err_msg: e.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?
            .iter()
            .map(|model| GetPostResponse {
                post_id: model.id,
                title: model.title.clone(),
                content: model.content.clone(),
                excerpt: model.excerpt.clone(),
            })
            .collect::<Vec<GetPostResponse>>();

        Ok(posts)
    }

    pub async fn delete_post(&self, post_id: i32) -> anyhow::Result<DeletePostResponse, AppError> {
        let post = posts::Entity::find_by_id(post_id)
            .one(&self._db_connection)
            .await
            .map_err(|e| AppError {
                err_msg: e.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?
            .context("could not find post id in database")
            .map_err(|e| AppError {
                err_msg: e.to_string(),
                status_code: StatusCode::BAD_REQUEST,
            })?;

        let _delete_res = post
            .delete(&self._db_connection)
            .await
            .map_err(|e| AppError {
                err_msg: e.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?;

        Ok(DeletePostResponse { post_id })
    }
}
