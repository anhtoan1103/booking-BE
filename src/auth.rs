use sqlx::PgPool;
use bcrypt::hash;
use sqlx::Row;
use sqlx::Error;
use bcrypt::verify;
use crate::auth_utils::jwt::{create_jwt, verify_jwt, Claims};
pub async fn verify_user(pool: &PgPool, email: &str, password: &str) -> Result<Option<String>, Error> {
    let row = sqlx::query("SELECT id, password from users where email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;
    
    let (user_id, stored_hash) = match row {
        Some(row) => (
            row.get::<String, _>("password"),
            row.get::<String, _>("password")
        ),
        None => return Ok(None),
    };
    if verify(password, &stored_hash).unwrap_or(false) {
        match create_jwt(&user_id.to_string(), 360000) {
            Ok(token) => {
                println!("JWT: {:?}", token);
                Ok(Some(token)) // Return the JWT token if verification is successful
            },
            Err(e) => {
                eprintln!("Error creating JWT: {:?}", e);
                Ok(None)
            }
        }
    } else {
        println!("Password verification failed");
        Ok(None) // Return None if password verification fails
    }
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

pub async fn verify_token(token: &str) -> Result<Claims, String> {
    verify_jwt(token)
}

