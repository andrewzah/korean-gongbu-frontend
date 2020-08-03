use warp::{self, Filter};

use crate::AppState;
use crate::handlers::{status_handler, user_handler};

pub fn routes(_state: AppState)
-> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::any()
        .and(warp::path("api")
            .and(warp::path("v1")
                .and(warp::path("users")
                    .and(warp::get())
                    .and_then(user_handler::index)
                )
                .or(warp::get()
                    .and(warp::path::param())
                    .and_then(status_handler::sleepy)
                )
            )
        )
}

pub fn with_state(state: AppState)
-> impl warp::Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || state.clone())
}

