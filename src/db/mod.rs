use sqlx::{PgPool, postgres::PgPoolOptions};

impl DB {
  pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
    let pool = PgPoolOptions::new()
      .max_connections(5) // You can adjust this as needed
      .connect(database_url)
      .await?;

    Ok(DB { pool })
  }

  pub fn pool(&self) -> &PgPool {
    &self.pool
  }
}

pub struct DB {
  pool: PgPool,
}
