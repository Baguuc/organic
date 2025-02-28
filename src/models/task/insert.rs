use sqlx::{query, query_as, PgConnection};

use super::Task;

pub struct TaskInsertQuery {
    title: String,
    description: String
}

impl TaskInsertQuery {
    pub fn new(title: String, description: String) -> Self {
        return Self {
            title,
            description
        };
    }

    pub async fn execute(self: &Self, db_conn: &mut PgConnection) -> Result<Task, sqlx::Error> {
        let sql = "
        INSERT INTO tasks (
            title,
            description,
            ordr,
            done
        )
        VALUES (
            $1,
            $2,
            COALESCE((SELECT ordr FROM tasks ORDER BY ordr DESC LIMIT 1), 0)+1,
            FALSE
        )
        RETURNING id;
        ";
        let result: (i32,) = query_as(sql)
            .bind(&self.title)
            .bind(&self.description)
            .fetch_one(db_conn)
            .await?;

        let task = Task {
            id: result.0,
            title: self.title.to_string(),
            description: self.description.to_string()
        };

        return Ok(task);
    }
}
