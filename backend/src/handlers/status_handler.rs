use std::time::Duration;

use tracing::{info, instrument};

use crate::errors::{EndpointResult};

#[instrument]
pub async fn sleepy(seconds: u64) -> EndpointResult {
    info!("test, seconds={}", seconds);

    tokio::time::delay_for(Duration::from_secs(seconds)).await;
    let response = String::from("{\"data\": \"ok!\"}");

    // let _test_path = Path::new("/tmp/a").canonicalize()
     //   .map_err(AppError::from)?;

    Ok(warp::reply::json(&response))
}

