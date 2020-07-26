use warp;

pub type AppResult = std::result::Result<warp::reply::Json, warp::Rejection>;

#[derive(Debug)]
pub enum AppError {
    IO(std::io::Error)
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
