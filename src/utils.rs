use bcrypt::{hash, DEFAULT_COST};

fn hash_password(plain: &str) -> String {
  hash(plain, DEFAULT_COST).expect("Failed to hash password");
}