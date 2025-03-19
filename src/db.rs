use dotenv::dotenv;
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use std::env;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

pub async fn connect_db() -> Result<Pool<Postgres>, sqlx::Error> {
    /*
    Simple function that returns the only one connection pool
    to avoid too much connections.
    */

    dotenv().ok(); // load environment variable from .env
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    match pool {
        Ok(pool) => {
            print!("Connected to database!");
            Ok(pool)
        }
        Err(e) => {
            print!("Cannot connect to database!");
            Err(e)
        }
    }
}
