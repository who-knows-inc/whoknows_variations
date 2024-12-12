use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::post;
use sqlx::{Error as SqlxError, PgPool};


use crate::models::user::User;
use crate::security::security::verify_password;

#[derive(Serialize, Deserialize)]
pub struct DeleteUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteUserRequest {
    pub email: String,
    pub password: String,
}

#[post(
    "/deleteUser",
    format = "application/json",
    data = "<delete_user_request>"
)]
pub async fn delete_user(
    delete_user_request: Json<DeleteUserRequest>,
    pool: &State<PgPool>,
    cookies: &CookieJar<'_>,
) -> Json<DeleteUserResponse> {
    let delete_user_request = delete_user_request.into_inner();
    println!("Attempting to delete user: {}", delete_user_request.email);
    // Acquire a connection from the pool
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire connection: {:?}", e);
            return Json(DeleteUserResponse {
                success: false,
                message: "Internal server error".to_string(),
            });
        }
    };

    // Query the database for the user
    let user_result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        delete_user_request.email
    )
    .fetch_one(&mut conn)
    .await;

    match user_result {
        Ok(user) => {
            if verify_password(&user.password, &delete_user_request.password) {
                // remove the user's cookie
                cookies.remove_private(Cookie::from("auth_token"));
                //    delete user from database
                sqlx::query!(
                    "DELETE FROM users WHERE email = $1",
                    delete_user_request.email
                )
                .execute(&mut conn)
                .await
                .unwrap();
                println!("User '{}' deleted successfully.", user.username);
                Json(DeleteUserResponse {
                    success: true,
                    message: "User deleted successfully".to_string(),
                })
            } else {
                println!(
                    "Authentication failed for user '{}': Invalid password.",
                    user.username
                );
                Json(DeleteUserResponse {
                    success: false,
                    message: "Invalid username or password".to_string(),
                })
            }
        }
        Err(SqlxError::RowNotFound) => {
            println!(
                "Authentication failed: User '{}' not found.",
                delete_user_request.email
            );
            Json(DeleteUserResponse {
                success: false,
                message: "Invalid username or password".to_string(),
            })
        }
        Err(e) => {
            eprintln!("Database error during authentication: {:?}", e);
            Json(DeleteUserResponse {
                success: false,
                message: "Internal server error".to_string(),
            })
        }
    }
}
