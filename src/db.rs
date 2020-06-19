use crate::conf::Conf;
use sqlx::postgres::PgPool;
use std::error;

pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn new(cnf: Conf) -> Result<Self, Box<dyn error::Error>> {
        let url = format!(
            "postgresql://{}:{}@{}:5432/{}?ssl=false",
            cnf.pg_user, cnf.pg_pass, cnf.pg_host, cnf.pg_db
        );
        let pool = PgPool::builder().max_size(3).build(&url).await?;
        Ok(Db { pool })
    }

    pub async fn get_data(&self) -> Result<u64, Box<dyn error::Error>> {
        let cursor = sqlx::query(
            "SELECT id from data_sources where is_curated = $1 or is_auto_curated = $1",
        )
        .bind(true)
        .fetch_all(&self.pool)
        .await?;
        let res = cursor.next().await?;
        println!("{}", res);
        Ok(res)
    }
}
