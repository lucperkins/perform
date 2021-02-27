use crate::bench::Bench;
use crate::Error;
use indoc::indoc;
use sqlx::{Pool, Postgres};

pub struct DB(Pool<Postgres>);

impl DB {
    pub async fn new(conn_url: &str) -> Result<Self, Error> {
        let db = Pool::connect(conn_url).await?;

        Ok(Self(db))
    }

    pub async fn create_bench_table(&self, id: usize) -> Result<(), Error> {
        let query = format!(
            indoc! {r#"
                CREATE TABLE IF NOT EXISTS benches_{} (
                    timestamp TIMESTAMP NOT NULL,
                    mean NUMERIC NOT NULL,
                    variance NUMERIC NOT NULL
                );
            "#},
            id
        );

        let _ = sqlx::query(&query).execute(&self.0).await?;

        Ok(())
    }

    pub async fn get_benches_by_id(&self, id: usize) -> Result<Vec<Bench>, Error> {
        let query = format!("SELECT * FROM benches_{}", id);
        let benches: Vec<Bench> = sqlx::query_as(&query).fetch_all(&self.0).await?;

        Ok(benches)
    }

    pub async fn add_bench(&self, id: usize, bench: Bench) -> Result<(), Error> {
        let query = format!(
            indoc! {r#"
                INSERT INTO benches_{} (timestamp, mean, variance)
                VALUES ($1, $2, $3)
            "#},
            id
        );
        let mut tx = self.0.begin().await?;
        let _ = sqlx::query(&query)
            .bind(bench.timestamp)
            .bind(bench.mean)
            .bind(bench.variance)
            .execute(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(())
    }
}
