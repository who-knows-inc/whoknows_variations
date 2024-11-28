#[cfg(test)]
mod tests {
    use rocket::{local::asynchronous::Client, http::Status, routes};
    use rocket::serde::json::json;
    use sqlx::{Executor, PgPool};
    use whoknows_nooneknows::routes::api::login::{login, LoginResponse};
    use whoknows_nooneknows::security::security::hash_password;

    async fn setup_test_db() -> PgPool {
        let db_url = "postgres://postgres:postgres@localhost:5432/postgres"; 
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
            "#
        )
        .await
        .expect("Failed to set up test database");

        pool
    }

    async fn insert_test_user(pool: &PgPool) {
        // Hash the password before inserting it
        let hashed_password = hash_password("password123");

        pool.execute(sqlx::query!(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
            "testuser",
            "testuser@example.com",
            hashed_password
        ))
        .await
        .expect("Failed to insert test user");
    }

    #[rocket::async_test]
    async fn test_login_success() {
        let pool = setup_test_db().await;
        insert_test_user(&pool).await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![login]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .post("/login")
            .header(rocket::http::ContentType::JSON)
            .body(json!({
                "username": "testuser",
                "password": "password123"
            }).to_string())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<LoginResponse>().await.unwrap();
        assert!(body.success, "Expected success but got failure");
        assert_eq!(body.message, "Login successful");
    }

  #[rocket::async_test]
    async fn test_login_failure_invalid_password() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![login]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .post("/login")
            .header(rocket::http::ContentType::JSON)
            .body(json!({
                "username": "testuser",
                "password": "wrongpassword"
            }).to_string())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<LoginResponse>().await.unwrap();
        assert!(!body.success);
        assert_eq!(body.message, "Invalid username or password");
    }

    #[rocket::async_test]
    async fn test_login_failure_user_not_found() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![login]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .post("/login")
            .header(rocket::http::ContentType::JSON)
            .body(json!({
                "username": "nonexistentuser",
                "password": "password123"
            }).to_string())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<LoginResponse>().await.unwrap();
        assert!(!body.success);
        assert_eq!(body.message, "Invalid username or password");
    }

    #[rocket::async_test]
    async fn test_login_failure_missing_fields() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![login]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .post("/login")
            .header(rocket::http::ContentType::JSON)
            .body(json!({
                "username": "testuser" // Missing "password" field
            }).to_string())
            .dispatch()
            .await;

        // Assert correct status
        assert_eq!(response.status(), Status::UnprocessableEntity);

        // Assert response body contains Rocket's default HTML
        let body = response.into_string().await.unwrap_or_default();
        assert!(
            body.contains("<title>422 Unprocessable Entity</title>"),
            "Expected HTML error page, got: {}",
            body
        );
    }
}