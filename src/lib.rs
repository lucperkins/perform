mod bench;
mod db;
mod results;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("sql error")]
    Sql(#[from] sqlx::Error),
}
