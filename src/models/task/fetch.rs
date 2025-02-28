use sqlx::{query_as, PgConnection};

use super::Task;

pub struct TaskFetchQuery {
    id: i32
}

impl TaskFetchQuery {
    pub fn new(id: i32) -> Self {
        return Self {
            id
        };
    }

    pub async fn execute(self: &Self, db_conn: &mut PgConnection) -> Result<Task, sqlx::Error> {
        let sql = "SELECT id, title, description, ordr, done FROM tasks WHERE id = $1;";
        let result = query_as(sql)
            .bind(&self.id)
            .fetch_one(db_conn)
            .await?;

        return Ok(result);
    }
}
