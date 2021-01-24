use diesel::prelude::*;
use tracing::debug;

use crate::{db::*, errors::AppError, models::v1::user::*};

#[tracing::instrument]
pub fn all() -> Result<Vec<User>, AppError> {
    use crate::schema::users::dsl::*;

    let conn = pool().get()?;
    let results = users.load::<User>(&conn)?;

    debug!("{} users found", results.len());

    Ok(results)
}
