use clap::Parser;
use common::UrchinConfig;
use std::{fs::File, io::Read, path::Path, sync::Arc};

use axum::{
    debug_handler, extract::Query, http::StatusCode, response::Html, routing::get, Extension, Json,
    Router,
};
use common::{AppError, Database, GetPostResponse};
use minijinja::Environment;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

// TODO : Rename this to something more useful
type DatabaseT = Arc<RwLock<Database>>;

#[derive(clap::Parser)]
struct ProgramArgs {
    // path to the config toml
    #[clap(long, short)]
    config_file: String,
}

#[derive(Serialize)]
struct IndexTemplateData {
    posts: Vec<GetPostResponse>,
}

fn read_file<T: AsRef<Path>>(path: T) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

async fn try_main() -> anyhow::Result<()> {
    // Read the config
    let args = ProgramArgs::parse();

    let config_contents = read_file(args.config_file)?;
    let config = toml::from_str::<UrchinConfig>(&config_contents)?;

    // TODO : we probably don't want to print secrets ;)
    println!("config");
    println!("{:#?}", config);

    let database = Arc::new(RwLock::new(
        Database::new(&config.database_address, config.database_port).await?,
    ));
    // Axum for multiplexing the http connections to entpoints
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(home_handler))
        .layer(Extension(database));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.webserver_port))
        .await
        .unwrap();
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

#[derive(Deserialize)]
struct HomeHandlerParams {
    page_num: i32,
}

#[debug_handler]
async fn home_handler(
    Extension(database_lock): Extension<DatabaseT>,
    Query(home_params): Query<HomeHandlerParams>,
) -> Result<Html<String>, Json<AppError>> {
    let database = database_lock.read().await;

    // TODO : We should probably check that the page_num
    // TODO : is a valid integer
    let posts = database.get_posts(home_params.page_num, 10).await?;

    // TODO : Pass the config for the path of the current views
    let html =
        read_file("/home/matheus/development/urchin-rs/views/index.html.in").map_err(|e| {
            AppError {
                err_msg: e.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    // TODO : create the template enviornment in main
    // TODO : and pass the template environment as an extension
    // TODO : to this function
    // Come in as a extension
    let mut env = Environment::new();
    env.add_template("index", &html).map_err(|_| AppError {
        err_msg: "could not parse template".into(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let tmpl = env.get_template("index").map_err(|_| AppError {
        err_msg: "could not get template".into(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let template = tmpl
        .render(IndexTemplateData {
            posts: posts.clone(),
        })
        .map_err(|_| AppError {
            err_msg: "could not render template".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Html(template))
}
