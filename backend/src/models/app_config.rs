use std::net::{SocketAddr, Ipv4Addr, IpAddr};

use dotenv;

use crate::errors::AppResult;

pub struct AppConfig {
    pub addr: SocketAddr,
}

impl AppConfig {
    pub fn new() -> AppResult<Self> {
        let port: u16 = dotenv::var("GONGBU_PORT")
            .expect("GONGBU_PORT is not set")
            .parse()?;
        let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

        Ok(AppConfig {
            addr: bind_address,
        })
    }
}