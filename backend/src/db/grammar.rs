use diesel::prelude::*;
use tracing::debug;

use crate::{db::*, errors::AppError};
use crate::models::v1::grammar::Grammar;

pub fn all() -> Result<Vec<Grammar>, AppError> {
    use crate::schema::grammars::dsl::*;

    let conn = pool().get()?;
    let results = grammars.load::<Grammar>(&conn)?;

    debug!("{} grammars found", results.len());
    Ok(results)
}
