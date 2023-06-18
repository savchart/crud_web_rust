mod data;
mod db;
mod error;
mod handler;

use std::convert::Infallible;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use warp::{http::StatusCode, Filter, Rejection};

type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;
type Result<T> = std::result::Result<T, warp::Rejection>;

impl warp::reject::Reject for error::Error {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract=(DBPool, ), Error=Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health").map(|| StatusCode::OK);
    let routes = health_route
        .with(warp::cors()
        .recover(error::handle_rejection));
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool).await.expect("database can be initialized");
    let todo = warp::path("todo");
    let todo_routes = todo
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(handler::create_todo_handler);

    let routes = health_route
        .or(todo_routes)
        .with(warp::cors())
        .recover(error::handle_rejection);
}
