use tracing::{info, Level};
use tracing_subscriber;
use warp::Filter;

mod errors;
mod handlers;
mod models;
mod routes;

use crate::errors::{AppResult};
use crate::models::{app_config::AppConfig, app_state::AppState};

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");


fn setup() {
    dotenv::dotenv().ok();

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
        .with(warp::log(APPLICATION_NAME));

    info!("Server listening at {}", &app_config.addr);
    warp::serve(routes).run(app_config.addr).await;

    Ok(())
}
