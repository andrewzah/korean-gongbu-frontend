use warp::{Rejection, Reply};

use crate::{db::grammar, errors::AppError};
use crate::models::v1::responses::api::ApiResponse;
use crate::models::v1::requests::grammar::GrammarSearchRequest;

pub(crate) async fn list() -> Result<impl Reply, Rejection> {
    let grammars = grammar::all().map_err(AppError::from)?;
    let resp = ApiResponse::Data(grammars);

    Ok(warp::reply::json(&resp))
}

pub(crate) async fn search(req: GrammarSearchRequest) -> Result<impl Reply, Rejection> {
    let grammars = grammar::search(req.name).map_err(AppError::from)?;
    let resp = ApiResponse::Data(grammars);

    Ok(warp::reply::json(&resp))
}
