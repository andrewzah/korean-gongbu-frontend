use warp::{self, Filter};

use crate::AppState;
use crate::handlers::status_handler;

pub fn routes(_state: AppState)
-> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::any()
        .and(warp::path("api")
            .and(warp::get())
            .and(warp::path::param())
            .and_then(status_handler::sleepy)
        )
}

pub fn with_state(state: AppState)
-> impl warp::Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || state.clone())
}

