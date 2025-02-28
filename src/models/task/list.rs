use sqlx::{query, query_as, PgConnection};

use super::Task;

pub struct TaskListQuery {
    with_id: Option<i32>,
    with_limit: Option<i32>,
    with_offset: Option<i32>
}

impl TaskListQuery {
    pub fn new(
            with_id: Option<i32>, 
            with_limit: Option<i32>,
            with_offset: Option<i32>
        ) -> Self {
        return Self {
            with_id,
            with_limit,
            with_offset
        };
    }

    pub async fn execute(self: &Self, db_conn: &mut PgConnection) -> Result<Vec<Task>, sqlx::Error> {
        let limit = self.with_limit.unwrap_or(10);
        let offset = self.with_offset.unwrap_or(0);

        if self.with_id.is_some() {
            let sql = "
            SELECT 
                id,
                title,
                description
            FROM
                tasks
            WHERE
                NOT done
                AND
                id = $1
            ORDER BY ordr
            LIMIT $2
            OFFSET $3;
            ";
            
            let result: Vec<Task> = query_as(sql)
                .bind(&self.with_id.unwrap())
                .bind(&limit)
                .bind(&offset)
                .fetch_all(db_conn)
                .await?;

            return Ok(result);
        } else {
            let sql = "SELECT 
                id,
                title,
                description
            FROM
                tasks
            WHERE
                NOT done
            ORDER BY ordr
            LIMIT $1
            OFFSET $2;";
            
            let result: Vec<Task> = query_as(sql)
                .bind(&limit)
                .bind(&offset)
                .fetch_all(db_conn)
                .await?;

            return Ok(result);
        }
    }
}
