use crate::{db, DBPool};
use warp::{http::StatusCode, reject, Rejection, Reply};
use crate::data::{TodoRequest, TodoResponse};

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    let db_con = db::get_db_con(&db_pool).await.map_err(|e|reject::custom(e))?;
    db.execute("SELECT 1", &[]).await.map_err(|e|reject::custom(DBQueryError(e)))?;

    Ok(StatusCode::OK)
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) = err.find::<Error>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = e.to_string();
    } else if let Some(e) = err.find::<reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = e.to_string();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }

    let json = warp::reply::json(&ErrorResponse { message });

    Ok(warp::reply::with_status(json, code))
}

pub async fn create_todo_handler(body: TodoRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::create_todo(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn list_todos_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let todos = db::fetch_todos(&db_pool, query.q)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &todos
            .into_iter()
            .map(|todo| TodoResponse::of(todo))
            .collect,
    ))
}

pub async fn update_todo_handler(id: i32, body: TodoUpdateRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&TodoResponse::of(
        db::update_todo(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_todo_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::delete_todo(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}