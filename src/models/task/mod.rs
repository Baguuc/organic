pub mod insert;

use serde::{Serialize, Deserialize};
use sqlx::{query, PgConnection};

#[derive(Serialize, Deserialize, Clone, Debug, )]
pub struct Task {
    id: i32,
    title: String,
    description: String,
}
