use actix_web::{HttpResponse, Responder, post, web};

use crate::schemas::auth::{
    LoginSchema,
    RegisterSchema,
};
use crate::config::db::DbPool;


#[post("/register")]
async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterSchema>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
            INSERT INTO users (name, email, password)
            VALUES (?, ?, ?)
        "#,
        body.name,
        body.email,
        body.password,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
async fn login(
    body: web::Json<LoginSchema>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
            SELECT * FROM users WHERE email = ? AND password = ?
        "#,
        body.email,
        body.password,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User logged in"),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}
