#[cfg(test)]
mod tests {
    use rocket::{local::asynchronous::Client, routes};
    use rocket::http::{Status, Cookie};
    use rocket::serde::json::json;
    use sqlx::{PgPool, Executor};
    use whoknows_nooneknows::routes::api::delete_user::{DeleteUserResponse, delete_user};
    use whoknows_nooneknows::security::security::hash_password;

async fn setup_test_db() -> PgPool {
    let db_url = "postgres://postgres:postgres@localhost:5432/postgres";
    let pool = PgPool::connect(db_url)
        .await
        .expect("Failed to connect to the database");

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );
        TRUNCATE TABLE users RESTART IDENTITY CASCADE;
        "#
    )
    .await
    .expect("Failed to set up test database");

    pool
}



   #[rocket::async_test]
async fn test_delete_user_success() {
    let pool = setup_test_db().await;

    // Insert unique user for this test
    let hashed_password = hash_password("password123");
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "unique_user",
        "unique_user@example.com",
        hashed_password
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test user");

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![delete_user]);

    let client = Client::tracked(rocket).await.unwrap();

    let response = client
        .post("/deleteUser")
        .header(rocket::http::ContentType::JSON)
        .cookie(Cookie::new("auth_token", "valid_token")) // Simulate an auth token
        .body(json!({
            "email": "unique_user@example.com",
            "password": "password123"
        }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_json::<DeleteUserResponse>().await.unwrap();
    assert!(
        body.success,
        "Expected success but got failure: {:?}",
        body.message
    );
    assert_eq!(body.message, "User deleted successfully");

    // Verify the user was deleted
    let user_count = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE email = $1", "unique_user@example.com")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user count");
    assert_eq!(user_count.count.unwrap(), 0, "User was not deleted");
}


 #[rocket::async_test]
async fn test_delete_user_invalid_password() {
    let pool = setup_test_db().await;

    // Insert user with a hashed password
    let hashed_password = hash_password("password123");
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "test_user",
        "test_user@example.com",
        hashed_password
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test user");

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![delete_user]);

    let client = Client::tracked(rocket).await.unwrap();

    let response = client
        .post("/deleteUser")
        .header(rocket::http::ContentType::JSON)
        .cookie(Cookie::new("auth_token", "valid_token")) // Simulate an auth token
        .body(json!({
            "email": "test_user@example.com",
            "password": "wrongpassword"
        }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_json::<DeleteUserResponse>().await.unwrap();
    assert!(
        !body.success,
        "Expected failure but got success: {:?}",
        body.message
    );
    assert_eq!(body.message, "Invalid username or password");

    // Ensure the user still exists
    let user_count = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE email = $1", "test_user@example.com")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user count");
    assert_eq!(user_count.count.unwrap(), 1, "User should not have been deleted");
}

    #[rocket::async_test]
    async fn test_delete_user_not_found() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![delete_user]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .post("/deleteUser")
            .header(rocket::http::ContentType::JSON)
            .cookie(Cookie::new("auth_token", "valid_token")) // Simulate an auth token
            .body(json!({
                "email": "nonexistent_user@example.com",
                "password": "password123"
            }).to_string())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<DeleteUserResponse>().await.unwrap();
        assert!(
            !body.success,
            "Expected failure but got success: {:?}",
            body.message
        );
        assert_eq!(body.message, "Invalid username or password");
    }
}