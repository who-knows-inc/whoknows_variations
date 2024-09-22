use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::{Error as SqlxError, PgPool};

use crate::security::security::verify_password;

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

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login_request>")]
pub async fn login(
    login_request: Form<LoginRequest>,
    pool: &State<PgPool>,
) -> Result<Redirect, Json<LoginResponse>> {
    // Get the login request from the form data
    let login_request = login_request.into_inner();

    // Acquire a connection from the pool
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire connection: {:?}", e);
            return Err(Json(LoginResponse {
                success: false,
                message: "Internal server error".to_string(),
            }));
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
                Ok(Redirect::to("/"))
            } else {
                // Password doesn't match
                Err(Json(LoginResponse {
                    success: false,
                    message: "Invalid username or password".to_string(),
                }))
            }
        }
        Err(SqlxError::RowNotFound) => {
            // User not found
            Err(Json(LoginResponse {
                success: false,
                message: "Invalid username or password".to_string(),
            }))
        }
        Err(e) => {
            // Other database error
            eprintln!("Database error: {:?}", e);
            Err(Json(LoginResponse {
                success: false,
                message: "Internal server error".to_string(),
            }))
        }
    }
}
