use warp::{self, Filter};

use crate::AppState;
use crate::handlers::status;

pub fn routes(_state: AppState)
-> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("api")
        .and(warp::path!("")
            .and(warp::get())
            .and(warp::body::json())
            .and_then(status::sleepy))
}

pub fn with_state(state: AppState)
-> impl warp::Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || state.clone())
}

