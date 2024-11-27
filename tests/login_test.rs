#[cfg(test)]
mod tests {
  
    use rocket::{local::asynchronous::Client, http::Status, routes};
    use rocket::serde::json::json;
    use sqlx::{Executor, PgPool};
    use tokio; // Tilføjet for #[tokio::test]
   // use whoknows_nooneknows::routes::api::login::LoginResponse;
//use crate::routes::api::login::login;
use whoknows_nooneknows::routes::api::login::{login, LoginResponse};

async fn setup_test_db() -> PgPool {
    let db_url = "postgres://postgres:postgres@localhost:5432/postgres"; // Replace with your actual database URL
    let pool = PgPool::connect(db_url)
        .await
        .expect("Failed to connect to the database");

    // Clear the table before inserting test data
    pool.execute(
        r#"
        TRUNCATE TABLE users RESTART IDENTITY CASCADE;
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        );
        INSERT INTO users (username, email, password)
        VALUES ('testuser', 'testuser@example.com', '$2b$12$ePxb7o6.4Emczjb1oRHDguXtzFQXBsq.QEEdv0IxOjNHEqC.pQF.O');
        "#,
    )
    .await
    .expect("Failed to set up test database");

    pool
}
#[tokio::test]
async fn test_login_success() {
    let pool = setup_test_db().await;

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![login]);

    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client
        .post("/login")
        .header(rocket::http::ContentType::JSON)
        .body(
            json!({
                "username": "testuser",
                "password": "password123" // Matches the bcrypt hash
            })
            .to_string(),
        )
        .dispatch()
        .await;

    // Assert correct status
    assert_eq!(response.status(), Status::Ok);

    // Assert response body
    let body: LoginResponse = response.into_json().await.unwrap();
    assert!(body.success);
    assert_eq!(body.message, "Login successful");
}




    #[tokio::test]
    async fn test_login_failure_invalid_password() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![login]);

        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client
            .post("/login")
            .header(rocket::http::ContentType::JSON)
            .body(
                json!({
                    "username": "testuser",
                    "password": "wrongpassword"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<LoginResponse>().await.unwrap();
        assert!(!body.success);
        assert_eq!(body.message, "Invalid username or password");
    }

    #[tokio::test]
    async fn test_login_failure_user_not_found() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![login]);

        let client = Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client
            .post("/login")
            .header(rocket::http::ContentType::JSON)
            .body(
                json!({
                    "username": "nonexistentuser",
                    "password": "password123"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<LoginResponse>().await.unwrap();
        assert!(!body.success);
        assert_eq!(body.message, "Invalid username or password");
    }

#[tokio::test]
async fn test_login_failure_missing_fields() {
    let pool = setup_test_db().await;

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![login]);

    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client
        .post("/login")
        .header(rocket::http::ContentType::JSON)
        .body(
            json!({
                "username": "testuser"
                // Missing "password" field
            })
            .to_string(),
        )
        .dispatch()
        .await;

    // Assert correct status
    assert_eq!(response.status(), Status::UnprocessableEntity);

    // No body expected for 422, so don't assert on it
    let body = response.into_string().await;
    assert!(body.is_none() || body.unwrap().is_empty());
}


}