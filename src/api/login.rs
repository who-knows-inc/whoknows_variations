use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>, pool: &State<PgPool>) -> Json<LoginResponse> {
    let login_request = login_request.into_inner();
    let mut conn = pool.acquire().await.unwrap();

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        login_request.username
    )
    .fetch_one(&mut conn)
    .await
    .unwrap();

    if user.password == login_request.password {
        Json(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        })
    } else {
        Json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
        })
    }
}
