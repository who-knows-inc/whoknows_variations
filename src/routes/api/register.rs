use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::PgPool;

use crate::models::user::User;
use crate::security::security::hash_password;
#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

#[derive(FromForm)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub password2: String,
    pub email: String,
}

#[post("/register", data = "<register_request>")]
pub async fn register(
    register_request: Form<RegisterRequest>,
    pool: &State<PgPool>,
) -> Result<Redirect, Json<RegisterResponse>> {
    // Get the register request from the form data
    let register_request = register_request.into_inner();

    // Check if the passwords match
    if register_request.password != register_request.password2 {
        return Err(Json(RegisterResponse {
            success: false,
            message: "Passwords do not match".to_string(),
        }));
    }
    // Acquire a connection from the pool
    let mut conn = pool.acquire().await.unwrap();

    // Query the database for the user
    let user = match sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 OR email = $2",
        register_request.username,
        register_request.email
    )
    .fetch_one(&mut conn)
    .await
    {
        Ok(user) => user,
        Err(_) => User {
            id: 0,
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
        },
    };
    // Check if the username is already taken
    if user.username == register_request.username {
        return Err(Json(RegisterResponse {
            success: false,
            message: "Username already taken".to_string(),
        }));
        // Check if the email is already taken
    } else if user.email == register_request.email {
        return Err(Json(RegisterResponse {
            success: false,
            message: "Email already taken".to_string(),
        }));
    } else {
        // Hash the password and insert it into the database
        let hashed_password = hash_password(&register_request.password);

        // Insert the user into the database
        sqlx::query!(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
            register_request.username,
            register_request.email,
            hashed_password
        )
        .execute(&mut conn)
        .await
        .unwrap();

        // Redirect the user to the home page
        Ok(Redirect::to("/"))
    }
}
