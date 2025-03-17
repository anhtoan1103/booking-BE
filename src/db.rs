use dotenv::dotenv;
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use std::env;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

pub async fn connect_db() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Kết nối database: {}", database_url);

    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(p) => {
            print!("connected");
            Ok(p)
        }
        Err(e) => {
            print!("cannot connect");
            Err(e)
        }
    }
}
