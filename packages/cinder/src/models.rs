use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
pub struct App {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewApp {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(App)]
pub struct Deploy {
    pub id: String,
    pub sha: String,
    pub executed_at: NaiveDateTime,
    pub app_id: String,
}
