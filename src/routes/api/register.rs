use crate::security::security::hash_password;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub password2: String,
    pub email: String,
}

#[post("/register", format = "application/json", data = "<register_request>")]
pub async fn register(
    register_request: Json<RegisterRequest>,
    pool: &State<PgPool>,
) -> Json<RegisterResponse> {
    let register_request = register_request.into_inner();
    println!(
        "Attempting registration for user: {}, with email: {}",
        register_request.username, register_request.email
    );
    // Check if the passwords match
    if register_request.password != register_request.password2 {
        println!("ERROR: Passwords do not match");
        return Json(RegisterResponse {
            success: false,
            message: "Passwords do not match".to_string(),
        });
    }

    // Acquire a connection from the pool
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire connection: {:?}", e);
            return Json(RegisterResponse {
                success: false,
                message: "ERROR: Database connection error".to_string(),
            });
        }
    };

    // Query the database for the user
    let user_exists = sqlx::query!(
        "SELECT username, email FROM users WHERE username = $1 OR email = $2",
        register_request.username,
        register_request.email
    )
    .fetch_optional(&mut conn)
    .await;

    match user_exists {
        Ok(Some(user)) => {
            if user.username == register_request.username {
                println!("ERROR: Username already taken");
                return Json(RegisterResponse {
                    success: false,
                    message: "Username already taken".to_string(),
                });
            } else if user.email == register_request.email {
                println!("ERROR: Email already taken");
                return Json(RegisterResponse {
                    success: false,
                    message: "Email already taken".to_string(),
                });
            }
        }
        Ok(None) => {}
        Err(_) => {
            println!("ERROR: Database error occurred");
            return Json(RegisterResponse {
                success: false,
                message: "Database error occurred".to_string(),
            });
        }
    }

    // Hash the password and insert it into the database
    let hashed_password = hash_password(&register_request.password);

    // Insert the user into the database
    match sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        register_request.username,
        register_request.email,
        hashed_password
    )
    .execute(&mut conn)
    .await
    {
        Ok(_) => {
            println!("Registration successful");
            Json(RegisterResponse {
                success: true,
                message: "Registration successful".to_string(),
            })
        }
        Err(_) => {
            println!("ERROR: Failed to create user");
            Json(RegisterResponse {
                success: false,
                message: "Failed to create user".to_string(),
            })
        }
    }
}
