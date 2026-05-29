use actix_web::{HttpResponse, Responder, post, web};

use crate::models::user::User;
use crate::schemas::auth::{
    LoginSchema,
    RegisterSchema,
};
use crate::config::db::DbPool;
use crate::utils::password::{hash_password, check_password};


#[post("/register")]
async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterSchema>,
) -> impl Responder {
    // check the email
    let user = sqlx::query_as::<_, User>(
        r#"
            SELECT * FROM users WHERE email = ?
        "#,
    )
    .bind(&body.email)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest()
                .body("User with this email already exists.");
        }
        Ok(None) => {
            let hashed_password = match hash_password(&body.password) {
                Ok(hash) => hash,
                Err(_) => {
                    return HttpResponse::InternalServerError()
                        .body("Failed to hash password");
                }
            };
            let result = sqlx::query!(
                r#"
                    INSERT INTO users (name, email, password)
                    VALUES (?, ?, ?)
                "#,
                body.name,
                body.email,
                hashed_password,
            )
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => {
                    return HttpResponse::Ok().body("User created");
                },
                Err(e) =>
                {
                    return HttpResponse::InternalServerError().body(e.to_string());
                },
            };
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to fetch user email: {}", e));
        }
    };
}

#[post("/login")]
async fn login(
    body: web::Json<LoginSchema>,
    pool: web::Data<DbPool>,
) -> impl Responder {

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT *
        FROM users
        WHERE email = ?
        "#
    )
    .bind(&body.email)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            match check_password(&body.password, &user.password) {
                Ok(true) => HttpResponse::Ok().body("Login successful"),
                _ => HttpResponse::Unauthorized().body("Invalid credentials"),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}
