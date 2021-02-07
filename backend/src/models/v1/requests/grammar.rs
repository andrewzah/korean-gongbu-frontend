use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GrammarSearchRequest {
    pub name: String,
}
