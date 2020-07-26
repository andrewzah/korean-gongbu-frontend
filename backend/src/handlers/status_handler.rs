use std::path::Path;
use std::time::Duration;

use crate::errors::{AppError, AppResult};

pub async fn sleepy(seconds: u64) -> AppResult {
    tokio::time::delay_for(Duration::from_secs(seconds)).await;
    let response = String::from("{\"data\": \"ok!\"}");

    // let _test_path = Path::new("/tmp/a").canonicalize()
     //   .map_err(AppError::from)?;

    Ok(warp::reply::json(&response))
}

