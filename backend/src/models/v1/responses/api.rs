use serde::Serialize;

use crate::errors::AppError;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiResponse<T> {
    Data(T),
    Errors(Vec<String>),
}

impl<T> ApiResponse<T> {
    pub fn err_from_err(e: &AppError) -> Self {
        ApiResponse::Errors(vec![e.to_string()])
    }

    pub fn err_from_str(msg: String) -> Self {
        ApiResponse::Errors(vec![msg])
    }
}
