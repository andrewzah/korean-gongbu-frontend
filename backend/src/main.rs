use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use warp::Filter;

mod errors;
mod handlers;
mod routes;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
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
}
