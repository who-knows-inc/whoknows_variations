use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::{Error as SqlxError, PgPool};

use crate::models::user::User;
use crate::security::security::verify_password;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

#[derive(FromForm)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<login_request>")]
pub async fn login(
    login_request: Form<LoginRequest>,
    pool: &State<PgPool>,
    cookies: &CookieJar<'_>,
) -> Json<LoginResponse> {
    // Get the login request from the form data
    let login_request = login_request.into_inner();

    // Acquire a connection from the pool
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

    // Query the database for the user
    let user_result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        login_request.username
    )
    .fetch_one(&mut conn)
    .await;

    match user_result {
        Ok(user) => {
            // Verify the password
            if verify_password(&user.password, &login_request.password) {
                println!("User found: {:?}", user.username);

                // Set a private cookie with the user's ID
                cookies.add(Cookie::new("user_id", user.id.to_string()));
                Json(LoginResponse {
                    success: true,
                    message: "Login successful".to_string(),
                })
            } else {
                // Password doesn't match
                Json(LoginResponse {
                    success: false,
                    message: "Invalid username or password".to_string(),
                })
            }
        }
        Err(SqlxError::RowNotFound) => {
            // User not found
            Json(LoginResponse {
                success: false,
                message: "Invalid username or password".to_string(),
            })
        }
        Err(e) => {
            // Other database error
            eprintln!("Database error: {:?}", e);
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
