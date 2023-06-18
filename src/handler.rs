use crate::{db, DBPool};
use warp::{http::StatusCode, reject, Rejection, Reply};

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply, Rejection> {
    let db_con = db::get_db_con(&db_pool).await.map_err(|e|reject::custom(e))?;
    db.execute("SELECT 1", &[]).await.map_err(|e|reject::custom(DBQueryError(e)))?;

    Ok(StatusCode::OK)
}