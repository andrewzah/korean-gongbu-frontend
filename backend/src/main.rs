#[macro_use]
extern crate diesel;

use tracing::{info, Level};
use tracing_subscriber;
use warp::Filter;

mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod schema;

use crate::{
    models::{app_config::AppConfig, app_state::AppState},
    errors::AppResult,
};

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

fn setup() {
    if let Ok(_) = std::env::var("GONGBU_PROD") {
        dotenv::dotenv().ok();
    } else {
        dotenv::from_filename(".env.dev").ok();
    }

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("no global subscriber has been set")
}

#[tokio::main]
async fn main() -> AppResult<()> {
    setup();
    let app_config = AppConfig::new()?;

    let app_state = AppState {
        jwt_secret: String::from("todo: changeme"),
    };

    let routes = routes::routes(app_state)
        .with(warp::log(APPLICATION_NAME))
        .with(warp::cors().allow_origin("http://0.0.0.0:5000"));

    info!("Server listening at {}", &app_config.addr);

    warp::serve(routes).run(app_config.addr).await;

    Ok(())
}
