use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use sqlx::{Error as SqlxError, PgPool};

use crate::models::user::User;
use crate::security::security::verify_password;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<login_request>")]
pub async fn login(
    login_request: Json<LoginRequest>,
    pool: &State<PgPool>,
    cookies: &CookieJar<'_>,
) -> Json<LoginResponse> {
    let login_request = login_request.into_inner();
    println!("Attempting login for user: {}", login_request.username);

    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire connection: {:?}", e);
            return Json(LoginResponse {
                success: false,
                message: "Internal server error".to_string(),
            });
        }
    };

    let user_result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        login_request.username
    )
    .fetch_one(&mut conn)
    .await;

    match user_result {
        Ok(user) => {
            if verify_password(&user.password, &login_request.password) {
                println!("User '{}' authenticated successfully.", user.username);
                cookies.add(Cookie::new("user_id", user.id.to_string()));
                Json(LoginResponse {
                    success: true,
                    message: "Login successful".to_string(),
                })
            } else {
                println!(
                    "Authentication failed for user '{}': Invalid password.",
                    user.username
                );
                Json(LoginResponse {
                    success: false,
                    message: "Invalid username or password".to_string(),
                })
            }
        }
        Err(SqlxError::RowNotFound) => {
            println!(
                "Authentication failed: User '{}' not found.",
                login_request.username
            );
            Json(LoginResponse {
                success: false,
                message: "Invalid username or password".to_string(),
            })
        }
        Err(e) => {
            eprintln!("Database error during authentication: {:?}", e);
            Json(LoginResponse {
                success: false,
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[derive(FromForm)]
pub struct LogoutRequest {
    pub username: String,
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Redirect {
    // Get the logout request from the form data

    // Remove the user's cookie
    cookies.remove(Cookie::from("user_id"));

    // Redirect the user to the home page
    Redirect::to("/")
}
