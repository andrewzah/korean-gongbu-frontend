use std::time::Duration;

use diesel::prelude::*;
use tracing::debug;

use crate::{
    db::user,
    models::user::*,
    errors::{AppError, EndpointResult},
};

#[tracing::instrument]
pub async fn index() -> EndpointResult {
    let users = user::all().map_err(AppError::from)?;

    Ok(warp::reply::json(&users))
}

