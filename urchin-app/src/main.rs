use std::sync::{Arc, RwLock};
use std::time::Duration;

use axum::routing::post;
use axum::{extract, Extension, Router};
use common::Database;
use sea_orm::ConnectOptions;
use serde::Deserialize;

// Data that user will pass to the endpoint
#[derive(Deserialize)]
struct AddPostRequest {
    title: String,
    content: String,
    excerpt: String,
}

// TODO : Rename this to something more useful
type DatabaseT = Arc<RwLock<Database>>;

async fn try_main() -> anyhow::Result<()>
{

    let database = Arc::new(RwLock::new(Database::new("0.0.0.0", 3336).await?));
    // Axum for multiplexing the http connections to entpoints
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/post", post(add_post_handler))
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
        },
        _ => {}
    }
}

async fn add_post_handler(
    Extension(database_lock): Extension<DatabaseT>,
    extract::Json(post_request) : extract::Json<AddPostRequest>,
) -> String {
    
    let database = database_lock.read().unwrap();
    let _ = database.add_post(&post_request.title, &post_request.content, &post_request.excerpt).await;


    match database_lock.read() {
        Ok(database) => {
            let mut output = String::from("Hello World");
            output.push_str(format!("\nDatabase IP: {:?}", database.ip).as_str());
            output.push_str(format!("\nDatabase Port: {}", database.port).as_str());
            output.push_str(format!("\nPost title: {}", post_request.title).as_str());
            output.push_str(format!("\nPost content: {}", post_request.content).as_str());
            output.push_str(format!("\nPost exceprt: {}", post_request.excerpt).as_str());
            
            return output;
        },
        _ => {
            return String::from("Didn't work");
        }
    }
}
