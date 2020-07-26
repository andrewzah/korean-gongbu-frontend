use dotenv;
use warp;

pub type AppResult<T> = std::result::Result<T, AppError>;
pub type EndpointResult = std::result::Result<warp::reply::Json, warp::Rejection>;

#[derive(Debug)]
pub enum AppError {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),

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
