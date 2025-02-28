use actix_web::{http::StatusCode, post, web::{Data, Json}, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::{models::task::insert::TaskInsertQuery, web::ServerResponse};

#[post("/tasks")]
pub async fn controller(
    task_query: Json<TaskInsertQuery>,
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
        Ok(task) => return ServerResponse::new(
            StatusCode::OK,
            Some(json!(task))
        ),
        Err(err) => return ServerResponse::new(
            StatusCode::BAD_REQUEST,
            Some(json!(err.to_string()))
        ) 
    }
}
