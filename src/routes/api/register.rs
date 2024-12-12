use crate::security::security::hash_password;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use sqlx::PgPool;
 use rocket::post;


#[derive(Serialize, Deserialize)]
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
        return Json(RegisterResponse {
            success: false,
            message: "Passwords do not match".to_string(),
        });
    }

    // Validate inputs
    if register_request.username.trim().is_empty() || register_request.password.trim().is_empty() {
        return Json(RegisterResponse {
            success: false,
            message: "Username and password cannot be empty".to_string(),
        });
    }
    if !register_request.email.contains('@') {
        return Json(RegisterResponse {
            success: false,
            message: "Invalid email format".to_string(),
        });
    }

    // Start transaction
    let mut transaction = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return Json(RegisterResponse {
                success: false,
                message: "Database transaction error".to_string(),
            });
        }
    };

    // Check if user exists
    match sqlx::query!(
        "SELECT username, email FROM users WHERE username = $1 OR email = $2",
        register_request.username,
        register_request.email
    )
    .fetch_optional(&mut transaction)
    .await
    {
        Ok(Some(user)) => {
            if user.username == register_request.username {
                return Json(RegisterResponse {
                    success: false,
                    message: "Username already taken".to_string(),
                });
            }
            if user.email == register_request.email {
                return Json(RegisterResponse {
                    success: false,
                    message: "Email already taken".to_string(),
                });
            }
        }
        Ok(None) => {} // No conflicts, continue
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            return Json(RegisterResponse {
                success: false,
                message: "Database query error".to_string(),
            });
        }
    }

    // Hash the password and insert it
    let hashed_password = hash_password(&register_request.password);
    println!(
        "Attempting to insert: username = {}, email = {}, password hash = {}",
        register_request.username, register_request.email, hashed_password
    );

    match sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        register_request.username,
        register_request.email,
        hashed_password
    )
    .execute(&mut transaction)
    .await
    {
        Ok(result) => {
            println!("Insert successful, rows affected: {}", result.rows_affected());
            transaction.commit().await.unwrap_or_else(|e| {
                eprintln!("Failed to commit transaction: {:?}", e);
            });
            Json(RegisterResponse {
                success: true,
                message: "Registration successful".to_string(),
            })
        }
        Err(e) => {
            eprintln!("ERROR: Failed to create user: {:?}", e);
            Json(RegisterResponse {
                success: false,
                message: "Failed to create user".to_string(),
            })
        }
    }
}
