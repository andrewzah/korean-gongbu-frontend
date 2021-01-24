pub(crate) mod status_routes;
pub(crate) mod user_routes;

use std::convert::Infallible;

use warp::{self, Filter};

use crate::AppState;

pub fn service_routes(
    _state: AppState,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any().and(user_routes::routes().or(status_routes::routes()))
}

#[allow(dead_code)]
pub fn with_state(
    state: AppState,
) -> impl warp::Filter<Extract = (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}
