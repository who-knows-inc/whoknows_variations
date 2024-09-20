use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::PgPool;

use crate::api::login::User;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[post("/register", data = "<register_request>")]
pub async fn register(
    register_request: Json<RegisterRequest>,
    pool: &State<PgPool>,
) -> Json<RegisterResponse> {
    let register_request = register_request.into_inner();
    let mut conn = pool.acquire().await.unwrap();

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        register_request.username
    )
    .fetch_one(&mut conn)
    .await
    .unwrap();

    if user.email == register_request.email {
        Json(RegisterResponse {
            success: false,
            message: "Username already taken".to_string(),
        })
    } else if user.username == register_request.username {
        Json(RegisterResponse {
            success: false,
            message: "Email already taken".to_string(),
        })
    } else {
        let user = RegisterRequest {
            username: register_request.username,
            password: register_request.password,
            email: register_request.email,
        };

        sqlx::query!(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
            user.username,
            user.email,
            user.password
        )
        .execute(&mut conn)
        .await
        .unwrap();

        Json(RegisterResponse {
            success: true,
            message: "User registered".to_string(),
        })
    }
}
