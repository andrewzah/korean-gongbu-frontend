use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    email: String,
    password_hash: String,
    account_type: i32,
}
