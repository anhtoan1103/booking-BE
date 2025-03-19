use sqlx::postgres::PgRow;
use sqlx::Error;
use sqlx::Row;
mod db;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// or #[tokio::main]
// or #[actix_web::main]
#[tokio::main] // Requires the `attributes` feature of `async-std`
async fn main() -> Result<(), Error> {
    // create pool connection
    let pool = match db::connect_db().await {
        Ok(pool) => pool,
        Err(error) => {
            eprintln!("cannot connect to db {:?}", error);
            std::process::exit(1);
        }
    };

    // read database using pool
    let email = "toto@gmail.com";

    let row: PgRow = sqlx::query("select password from users where email = $1")
        .bind(email)
        .fetch_one(&pool)
        .await?;
    let row1: PgRow = sqlx::query("select password from users where email = $1")
        .bind(email)
        .fetch_one(&pool)
        .await?;
    let password: String = row.get("password");
    let password1: String = row1.get("password");
    println!("{}", password);
    println!("{}", password1);
    Ok(())
}
