use sqlx::postgres::PgRow;
use sqlx::Error;
use sqlx::PgPool;
use sqlx::Row;
mod db;
mod auth;
mod models;
use std::sync::Arc;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[post("/login")]
async fn login(pool: web::Data<Arc<PgPool>>, user: web::Json<models::UserLogin>) -> HttpResponse {
    match auth::verify_user(&pool, &user.email, &user.password).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({"message": "Login successful"})),  // ✅ Login success
        Ok(false) => HttpResponse::Unauthorized().json(serde_json::json!({"error": "Invalid credentials"})),  // ❌ Wrong password/email
        Err(e) => {
            eprintln!("🚨 Database error: {:?}", e);  // ✅ Log the error for debugging
            HttpResponse::InternalServerError().json(serde_json::json!({"error": "Something went wrong"}))  // ❌ DB error
        }
    }
}


#[post("/create")]
async fn create(pool: web::Data<Arc<PgPool>>, user: web::Json<models::User>) -> impl Responder {
    match auth::create_user(&pool, &user.email, &user.password).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "User created successfully" })),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Failed to create user" }))
        }
    }
}

// or #[tokio::main]
// or #[actix_web::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // create pool connection
    let pool = Arc::new(db::connect_db().await.expect("Failed to connect to DB"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(login)
            .service(create)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
