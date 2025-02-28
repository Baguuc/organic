use actix_web::{get, http::StatusCode, web::{Data, Json, Query}, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::{models::task::list::TaskListQuery, web::ServerResponse};

#[get("/tasks")]
pub async fn controller(
    task_query: Query<TaskListQuery>,
    db_conn: Data<PgPool>
) -> impl Responder {
    let mut db_conn = db_conn
        .acquire()
        .await
        .unwrap();

    let task = task_query
        .execute(&mut db_conn)
        .await;

    match task {
        Ok(tasks) => return ServerResponse::new(
            StatusCode::OK,
            Some(json!(tasks))
        ),
        Err(err) => return ServerResponse::new(
            StatusCode::BAD_REQUEST,
            Some(json!(err.to_string()))
        ) 
    }
}
