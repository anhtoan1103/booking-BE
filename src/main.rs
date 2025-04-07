use sqlx::PgPool;
use actix_cors::Cors;
use dotenv::dotenv;
mod db;
mod auth;
mod auth_utils;
mod models;
use actix_web::http::header;
use std::sync::Arc;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[post("/login")]
async fn login(pool: web::Data<Arc<PgPool>>, user: web::Json<models::UserLogin>) -> HttpResponse {
    match auth::verify_user(&pool, &user.email, &user.password).await {
        Ok(Some(token)) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Login successful",
            "token": token  // ✅ Return JWT token
        })),  // ✅ Login success
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })),  // ❌ Wrong password/email
        Err(e) => {
            eprintln!("🚨 Database error: {:?}", e);  // ✅ Log the error for debugging
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Something went wrong"
            }))  // ❌ DB error
        }
    }
}

#[post("/verify")]
async fn verify_jwt(token: web::Json<models::TokenPayload>) -> HttpResponse {
    match auth::verify_token(&token.token).await {
        Ok(claims) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Token is valid",
            "claims": claims  // ✅ Return decoded claims
        })),
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid or expired token"
        })),
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
    dotenv().ok();
    // create pool connection
    let pool = Arc::new(db::connect_db().await.expect("Failed to connect to DB"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // 👈 Cho phép frontend
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(login)
            .service(verify_jwt)
            .service(create)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
