use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::handlers::v1::status_handler;

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any().and(status().or(status()))
}

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "status" / ..).boxed()
}

fn status() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path_prefix())
        .and_then(status_handler::status)
}
