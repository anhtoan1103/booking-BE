use sqlx::PgPool;
use bcrypt::hash;
use sqlx::Row;
use sqlx::Error;
use bcrypt::verify;
pub mod jwt;
pub async fn verify_user(pool: &PgPool, email: &str, password: &str) -> Result<bool, Error> {
    let row = sqlx::query("SELECT password from users where email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;
    
    let stored_hash = match row {
        Some(row) => row.get::<String, _>("password"),
        _ => return Ok(false),
    };
    let test = create_jwt(1, 1000);

    Ok(verify(password, &stored_hash).unwrap_or(false))
}

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<(), Error> {
    let hashed_password = hash(password, 12).unwrap();
    sqlx::query("INSERT INTO users(email, password) VALUES ($1, $2)")
        .bind(email)
        .bind(hashed_password)
        .execute(pool)
        .await?;
    Ok(())
}
