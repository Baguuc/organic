use sqlx::{query, PgConnection};

use super::Task;

pub struct TaskDeleteQuery {
    id: i32
}

impl TaskDeleteQuery {
    pub fn new(id: i32) -> Self {
        return Self {
            id
        };
    }

    pub async fn execute(self: &Self, db_conn: &mut PgConnection) -> Result<(), sqlx::Error> {
        let sql = "DELETE FROM tasks WHERE id = $1;";
        let result = query(sql)
            .bind(&self.id)
            .execute(db_conn)
            .await?;

        return Ok(());
    }
}
