use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: Option<String>,
    pub pinned: bool,
    pub date_created: Date,
}
