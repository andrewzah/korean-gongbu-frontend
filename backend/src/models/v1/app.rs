use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
}

pub struct AppConfig {
    pub addr: SocketAddr,
}

impl AppConfig {
    pub fn new() -> Self {
        let port: u16 = dotenv::var("GONGBU_PORT")
            .expect("GONGBU_PORT is not set")
            .parse()
            .expect("Unable to parse GONGBU_PORT!");

        let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

        AppConfig { addr: bind_address }
    }
}
