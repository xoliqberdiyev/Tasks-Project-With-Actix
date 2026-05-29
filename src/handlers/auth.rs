use actix_web::{HttpResponse, Responder, post, web};
use crate::models::user::User;
use crate::schemas::auth::{
    LoginSchema,
    RegisterSchema,
};
use crate::config::db::DbPool;
use crate::config::config::Config;
use crate::utils::password::{hash_password, check_password};
use crate::utils::jwt::generate_jwt;


#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterSchema,
    responses(
        (status = 200, description = "User created"),
        (status = 400, description = "User already exists")
    )
)]
#[post("/register")]
pub async fn register(
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


#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginSchema,
    responses(
        (status = 200, description = "User logged in"),
        (status = 401, description = "Invalid credentials")
    )
)]
#[post("/login")]
pub async fn login(
    body: web::Json<LoginSchema>,
    pool: web::Data<DbPool>,
    cnf: web::Data<Config>,
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
                Ok(true) => {
                    let token = generate_jwt(user.id as u64, cnf.get_ref());
                    HttpResponse::Ok().json(serde_json::json!(
                        {
                            "token": token,
                            "token_type": "Bearer",
                            "user_id": user.id,
                        }
                    ))
                },
                _ => HttpResponse::Unauthorized().json(serde_json::json!(
                    {
                        "success": false,
                        "message": "Invalid credentials"
                    }
                )),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!(
            {
                "success": false,
                "message": "Invalid credentials or user not found"
            }
        )),
    }
}
