use env::var;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct DB {
  pool: PgPool,
}

impl DB {
  pub async fn new() -> Result<Self, sqlx::Error> {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
      .max_connections(
        var("MAX_CONNECTIONS")
          .unwrap_or("5".to_string())
          .parse()
          .unwrap(),
      )
      .connect(&database_url)
      .await?;
    Ok(DB { pool })
  }

  pub fn pool(&self) -> &PgPool {
    &self.pool
  }
}
