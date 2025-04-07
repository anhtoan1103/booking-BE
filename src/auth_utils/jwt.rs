use serde::{Serialize, Deserialize};
use std::env;
use once_cell::sync::Lazy;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use jsonwebtoken::errors::Error;
use chrono::Utc;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  company: String,
  exp: usize,
}

static SECRET: Lazy<String> = Lazy::new(|| {
  env::var("SECRET").expect("SECRET must be set.")
});

static COMPANY_NAME: Lazy<String> = Lazy::new(|| {
  env::var("COMPANY_NAME").expect("COMPANY_NAME must be set.")
});

// we use &str here because user_id is unchangable. We add & to 
pub fn create_jwt(user_id: &str, exp: usize)
  -> Result<String, Error> {
  // dotenv().ok(); // load environment variable from .env
  let expiration = (Utc::now().timestamp() as usize) + exp;

  let my_claims = Claims
    {
      sub: user_id.to_owned(),
      company: COMPANY_NAME.to_owned(),
      exp: expiration,
    };
  
  let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(SECRET.as_bytes()))?;

  Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
  match decode::<Claims>(
    token,
    &DecodingKey::from_secret(SECRET.as_bytes()),
    &Validation::new(Algorithm::HS256),
  ) {
    Ok(token_data) => {
      println!("Token data: {:?}", token_data.claims);
      if token_data.claims.exp < Utc::now().timestamp() as usize {
        println!("Token expired");
        Err("Token expired".to_string())
      } else {
        println!("Token is valid");
        Ok(token_data.claims)
      }
    },
    Err(e) => {
      println!("Error decoding token: {:?}", e);
      Err("Invalid token".to_string())
    }
  }
}