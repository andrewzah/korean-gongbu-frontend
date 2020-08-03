use dotenv;

use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;
pub type EndpointResult = std::result::Result<warp::reply::Json, warp::Rejection>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error.")]
    IO(std::io::Error),
    #[error("Unable to parse int.")]
    ParseInt(std::num::ParseIntError),
    #[error("Error with r2d2 pool.")]
    R2D2(r2d2::Error),
    #[error("Error with diesel query.")]
    Diesel(diesel::result::Error),

    #[error("Unable to load env var.")]
    Env(dotenv::Error),
}

impl warp::reject::Reject for AppError {}

impl From<AppError> for warp::reject::Rejection {
    fn from(e: AppError) -> Self {
        warp::reject::custom(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IO(e)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::ParseInt(e)
    }
}

impl From<dotenv::Error> for AppError {
    fn from(e: dotenv::Error) -> Self {
        AppError::Env(e)
    }
}
impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        AppError::R2D2(e)
    }
}
impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        AppError::Diesel(e)
    }
}
