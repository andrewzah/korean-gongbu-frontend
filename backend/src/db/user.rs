use diesel::prelude::*;
use tracing::debug;

use crate::db::*;
use crate::models::user::*;
use crate::errors::AppResult;

#[tracing::instrument]
pub fn all() -> AppResult<Vec<User>> {
    use crate::schema::users::dsl::*;

    let conn = pool().get()?;
    let results = users.load::<User>(&conn)?;

    debug!("{} users found", results.len());

    Ok(results)
}
