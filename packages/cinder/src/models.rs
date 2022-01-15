use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct App {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewApp {
    pub name: String,
}
