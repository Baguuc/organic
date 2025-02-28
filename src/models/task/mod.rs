pub mod list;
pub mod fetch;
pub mod insert;
pub mod delete;
pub mod update;

use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow, query, PgConnection};

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Task {
    id: i32,
    title: String,
    description: String,
    ordr: i32,
    done: bool
}
