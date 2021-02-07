use diesel::prelude::*;
use tracing::debug;

use crate::{db::*, errors::AppError};
use crate::models::v1::grammar::Grammar;

pub(crate) fn all() -> Result<Vec<Grammar>, AppError> {
    use crate::schema::grammars::dsl::*;

    let conn = pool().get()?;
    let results = grammars.load::<Grammar>(&conn)?;

    debug!("{} grammars found", results.len());
    Ok(results)
}

pub(crate) fn search(query: String) -> Result<Vec<Grammar>, AppError> {
    use crate::schema::grammars::dsl::*;

    let conn = pool().get()?;
    let results = grammars
        .filter(name.like(query))
        .load::<Grammar>(&conn)?;

    debug!("{} grammars found", results.len());
    Ok(results)
}
