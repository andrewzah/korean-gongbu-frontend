#[macro_use]
extern crate diesel;

use std::env;

use http::StatusCode;
use tracing::{error, info};
use warp::{reply, Filter, Rejection, Reply};

mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod schema;

use crate::{
    errors::AppError,
    models::v1::{
        app::{AppConfig, AppState},
        responses::api::ApiResponse,
    },
    routes::v1::service_routes,
};

fn setup() {
    if std::env::var("GONGBU_PROD").is_ok() {
        dotenv::dotenv().ok();
    } else {
        dotenv::from_filename(".env.dev").ok();
    }

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "korean_gongbu,kg_warp,warp=info");
    }
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    setup();
    let app_config = AppConfig::new();

    let app_state = AppState {
        jwt_secret: String::from("todo: changeme"),
    };

    let routes = service_routes(app_state)
        .with(warp::log("kg_warp"))
        .with(warp::cors().allow_origin("http://0.0.0.0:5000"))
        .recover(handle_rejection);

    info!("Server listening at {}", &app_config.addr);
    warp::serve(routes).run(app_config.addr).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let (err_response, code) = match err.find::<AppError>() {
        Some(service_err) => {
            error!("{}", service_err.to_string());
            (
                ApiResponse::<()>::err_from_err(service_err),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
        None => {
            let msg = format!("{:?}", err);
            error!("{}", msg);

            (
                ApiResponse::<()>::err_from_str(msg),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    };

    Ok(reply::with_status(reply::json(&err_response), code))
}
