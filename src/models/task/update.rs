use sqlx::{query, PgConnection};

use super::{fetch::TaskFetchQuery, Task};

pub struct TaskUpdateQuery {
    id: i32,
    new_title: Option<String>,
    new_description: Option<String>,
    new_ordr: Option<i32>,
    new_done: Option<bool>
}

impl TaskUpdateQuery {
    pub fn new(id: i32, new_title: Option<String>, new_description: Option<String>, new_ordr: Option<i32>, new_done: Option<bool>) -> Self {
        return Self {
            id,
            new_title,
            new_description,
            new_ordr,
            new_done
        };
    }

    pub async fn execute(self: &Self, db_conn: &mut PgConnection) -> Result<(), sqlx::Error> {
        let task = TaskFetchQuery::new(self.id)
            .execute(db_conn)
            .await?;

        let new_title = self.new_title.clone().unwrap_or(task.title);
        let new_description = self.new_description.clone().unwrap_or(task.description);
        let new_ordr = self.new_ordr.unwrap_or(task.ordr);
        let new_done = self.new_done.unwrap_or(task.done);

        let sql = "UPDATE tasks SET title = $1, description = $2, ordr = $3, done = $4 WHERE id = $5;";
        let _ = query(sql)
            .bind(&new_title)
            .bind(&new_description)
            .bind(&new_ordr)
            .bind(&new_done)
            .bind(&self.id)
            .execute(db_conn)
            .await?;

        return Ok(());
    }
}
