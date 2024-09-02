use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::{debug_handler, extract, Extension, Json, Router};
use common::{AddPostRequest, AddPostResponse, AppError, Database, DeletePostResponse, GetPostResponse};
use http::StatusCode;
use tokio::sync::RwLock;

// TODO : Rename this to something more useful
type DatabaseT = Arc<RwLock<Database>>;

async fn try_main() -> anyhow::Result<()> {
    let database = Arc::new(RwLock::new(Database::new("0.0.0.0", 3336).await?));
    // Axum for multiplexing the http connections to entpoints
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/posts", post(add_post_handler))
        .route("/posts/:id", get(get_post_handler))
        .route("/posts/:id", delete(delete_post_handler))
        .layer(Extension(database));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    match try_main().await {
        Err(e) => {
            println!("exited program, error: {:?}", e);
        }
        _ => {}
    }
}

#[debug_handler]
async fn add_post_handler(
    Extension(database_lock): Extension<DatabaseT>,
    extract::Json(post_request): extract::Json<AddPostRequest>,
) -> Result<Json<AddPostResponse>, Json<AppError>> {
    // Check that everything is actually populated
    if post_request.title.is_empty() {
        // return some app error
        return Err(Json(AppError {
            err_msg: "cannot have empty post title".into(),
            status_code: StatusCode::BAD_REQUEST,
        }));
    }

    if post_request.excerpt.is_empty() {
        // return some app error
        return Err(Json(AppError {
            err_msg: "cannot have empty post excerpt".into(),
            status_code: StatusCode::BAD_REQUEST,
        }));
    }

    if post_request.content.is_empty() {
        // return some app error
        return Err(Json(AppError {
            err_msg: "cannot have empty post content".into(),
            status_code: StatusCode::BAD_REQUEST,
        }));
    }

    let database = database_lock.read().await;

    let post_id = match database
        .add_post(
            &post_request.title,
            &post_request.content,
            &post_request.excerpt,
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            return Err(Json(AppError {
                err_msg: format!("could not store post in db: {}", e),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }))
        }
    };

    Ok(Json(AddPostResponse { post_id }))
}

#[debug_handler]
async fn get_post_handler(
    Extension(database_lock): Extension<DatabaseT>,
    extract::Path(post_id): extract::Path<i32>,
) -> Result<Json<GetPostResponse>, AppError> {

    let database = database_lock.read().await;
    let post = database.get_post(post_id).await?;

    Ok(Json(post))
}

#[debug_handler]
async fn delete_post_handler(
    Extension(database_lock): Extension<DatabaseT>,
    extract::Path(post_id): extract::Path<i32>,
) -> Result<Json<DeletePostResponse>, AppError> {

    let database = database_lock.read().await;
    let post = database.delete_post(post_id).await?;

    Ok(Json(post))
}
