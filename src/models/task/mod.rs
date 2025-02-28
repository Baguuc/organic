pub mod list;
pub mod fetch;
pub mod insert;
pub mod delete;

use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow, query, PgConnection};

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Task {
    id: i32,
    title: String,
    description: String,
}
