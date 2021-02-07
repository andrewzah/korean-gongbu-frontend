use warp::{filters::BoxedFilter, path, Filter, Rejection, Reply};

use crate::models::v1::requests::grammar::*;
use crate::handlers::v1::grammar_handler;

pub(crate) fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any().and(search().or(list()))
}

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "v1" / "grammars" / ..).boxed()
}

fn list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get().and(path_prefix()).and_then(grammar_handler::list)
}

fn search_json_body() -> impl Filter<Extract = (GrammarSearchRequest,), Error = Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 48).and(warp::body::json())
}

fn search() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(path_prefix())
        .and(path("search"))
        .and(search_json_body())
        .and_then(grammar_handler::search)
}
