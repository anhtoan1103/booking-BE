async fn verify_user(PgPoolOptions: pool, String: email, String: pass) -> bool {
    check = sqlx::query_as("SELECT * from users where email = $1")
        .bind(email)
        .fetch_one(pool)
        .await;
    if check {
        return true;
    }
    return false;
}
