use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::handlers::v1::user_handler;

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any().and(get().or(get()))
}

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "users" / ..).boxed()
}

fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get().and(path_prefix()).and_then(user_handler::list)
}
