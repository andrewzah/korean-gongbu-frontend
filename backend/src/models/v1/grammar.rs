use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Grammar {
    pub id: i32,
    pub name: String,
    pub meaning_en: String,
}
