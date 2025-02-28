use actix_web::{delete, http::StatusCode, web::{Data, Path}, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::{models::task::{delete::TaskDeleteQuery, list::TaskListQuery}, web::ServerResponse};

#[delete("/tasks/{task_id}")]
pub async fn controller(
    task_id: Path<i32>,
    db_conn: Data<PgPool>
) -> impl Responder {
    let mut db_conn = db_conn
        .acquire()
        .await
        .unwrap();
    
    let task_id = task_id.into_inner();
    let task_query = TaskDeleteQuery::new(task_id);

    let task = task_query
        .execute(&mut db_conn)
        .await;

    match task {
        Ok(tasks) => return ServerResponse::new(
            StatusCode::OK,
            None
        ),
        Err(err) => return ServerResponse::new(
            StatusCode::BAD_REQUEST,
            Some(json!(err.to_string()))
        ) 
    }
}
