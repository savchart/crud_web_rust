use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{Config, Error, NoTls};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use mobc::Pool;

type Result<T> = std::result::Result<T, error::Error>;

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