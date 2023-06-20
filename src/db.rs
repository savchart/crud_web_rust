use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use mobc::Pool;

type Result<T> = std::result::Result<T, error::Error>;
const TABLE_NAME: &str = "todo";

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT: u64 = 10;
const INIT_SQL: &str = "./db.sql";

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://postgres:postgres@127.0.0.1:7878/postgres")?;
    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT)))
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .build(manager))
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str()).await.map_err(DBInitError);
    Ok(())
}

pub async fn create_todo(db_pool: &DBPool, body: TodoRequest) -> Result<Todo>{
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_todo(&row))
}

fn row_to_todo(row: &Row) ->Todo{
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let checked: bool = row.get(3);
    Todo {
        id,
        name,
        created_at,
        checked,
    }
}