use warp::{Rejection, Reply};

use crate::{db::user, errors::AppError};

pub async fn list() -> Result<impl Reply, Rejection> {
    let users = user::all().map_err(AppError::from)?;

    Ok(warp::reply::json(&users))
}
