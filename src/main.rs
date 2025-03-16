use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// or #[tokio::main]
// or #[actix_web::main]
#[tokio::main] // Requires the `attributes` feature of `async-std`
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Kết nối database: {}", database_url);

    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 50);

    Ok(())
}
