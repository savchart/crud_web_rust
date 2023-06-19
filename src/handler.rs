use crate::{db, DBPool};
use warp::{http::StatusCode, reject, Rejection, Reply};

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