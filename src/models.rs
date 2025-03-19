use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct User {
  pub id: Option<i32>,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
  pub email: String,
  pub password: String,
}
