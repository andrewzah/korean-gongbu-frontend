use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // ----- std -----
    #[error("Unable to load env var.")]
    Env(#[from] dotenv::Error),

    #[error("IO error.")]
    IO(#[from] std::io::Error),

    #[error("Unable to parse int.")]
    ParseInt(#[from] std::num::ParseIntError),

    // ----- 3rd party libs -----
    #[error("Error with r2d2 pool.")]
    R2D2(#[from] r2d2::Error),

    #[error("Error with diesel query.")]
    Diesel(#[from] diesel::result::Error),
}

impl warp::reject::Reject for AppError {}
