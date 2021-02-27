use crate::bench::Bench;
use indoc::indoc;
use sqlx::{Pool, Postgres};

pub struct DB(Pool<Postgres>);

impl DB {
    pub async fn new(conn_url: &str) -> anyhow::Result<Self> {
        let db = Pool::connect(conn_url).await?;

        Ok(Self(db))
    }

    pub async fn get_bench_by_name(&self, name: String) -> anyhow::Result<Option<Bench>> {
        let query = "SELECT * FROM benches WHERE name = $1";
        let bench = sqlx::query_as(query)
            .bind(name)
            .fetch_optional(&self.0)
            .await?;

        Ok(bench)
    }

    pub async fn create_bench(&self, bench: Bench) -> anyhow::Result<()> {
        let query = indoc! {r#"
            INSERT INTO benches (name, metadata, results)
            VALUES ($1, $2, $3)
        "#};
        let mut tx = self.0.begin().await?;
        let _ = sqlx::query(query)
            .bind(bench.name)
            .bind(bench.metadata)
            .bind(bench.results)
            .execute(&mut tx)
            .await?;

        let _ = tx.commit().await?;

        Ok(())
    }
}
