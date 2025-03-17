use tokio::time::sleep;
use tokio::time::Duration;

mod db;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

// or #[tokio::main]
// or #[actix_web::main]
#[tokio::main] // Requires the `attributes` feature of `async-std`
async fn main() {
    loop {
        match db::connect_db().await {
            Ok(..) => {
                break;
            }
            Err(error) => {
                eprintln!("cannot connect to db {:?}", error);
                sleep(Duration::from_millis(1000)).await;
            }
        }
    }
}
