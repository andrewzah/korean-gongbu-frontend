use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use warp::{Rejection, Reply};

use crate::models::v1::responses::api::ApiResponse;

#[derive(Serialize)]
struct HealthResponse {
    timestamp: String,
}

pub async fn status() -> Result<impl Reply, Rejection> {
    let res_data = HealthResponse {
        timestamp: format!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH)),
    };

    let response = ApiResponse::Data(res_data);
    Ok(warp::reply::json(&response))
}
