mod bench;
mod db;
mod results;

pub use db::DB;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty results set")]
    EmptyResults,

    #[error("sql error")]
    Sql(#[from] sqlx::Error),
}
