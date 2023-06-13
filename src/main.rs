mod data;
mod db;
mod error;
mod handler;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;

type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health").map(|| StatusCode::OK);
    let routes = health_route.with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool).await.expect("database can be initialized");
}
