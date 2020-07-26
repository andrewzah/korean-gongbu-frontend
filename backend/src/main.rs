use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tracing::{info, Level};
use tracing_subscriber;
use warp::Filter;

mod errors;
mod handlers;
mod routes;

use crate::errors::{AppResult, EndpointResult};

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
}

fn setup() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("no global subscriber has been set");
}

#[tokio::main]
async fn main() -> AppResult<()> {
    setup();

    let bind_address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3030
    );

    let app_state = AppState {
        jwt_secret: String::from("changeme"),
    };

    let routes = routes::routes(app_state)
        .with(warp::log(APPLICATION_NAME));

    println!("Server listening at {}", &bind_address);
    warp::serve(routes).run(bind_address).await;

    Ok(())
}
