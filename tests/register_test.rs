use rocket::{local::asynchronous::Client, routes};
use rocket::serde::json::json;
use sqlx::{PgPool, Executor};
use rocket::http::Status;
use whoknows_nooneknows::routes::api::register::{register, RegisterResponse};

async fn setup_test_db() -> PgPool {
    let db_url = "postgres://postgres:postgres@localhost:5432/postgres"; // Replace with your actual database URL
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
        TRUNCATE TABLE users;
        "#
    )
    .await
    .expect("Failed to set up test database");

    pool
}


#[rocket::async_test]
async fn test_register_success() {
    let pool = setup_test_db().await;

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![register]);

    let client = Client::tracked(rocket).await.unwrap();

    let response = client
        .post("/register")
        .header(rocket::http::ContentType::JSON)
        .body(json!({
            "username": "test_user33", // Ændrer brugernavn for at undgå tidligere konflikter
            "password": "password123",
            "password2": "password123", // Sørger for at passwords matcher
            "email": "test_user33@example.com"
        }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_json::<RegisterResponse>().await.unwrap();
    assert!(
        body.success,
        "Expected success but got failure: {:?}",
        body.message
    );
    assert_eq!(body.message, "Registration successful");

    // Kontroller, at brugeren blev oprettet korrekt
    let user_count = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE username = $1", "test_user33")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user count");
    assert_eq!(
        user_count.count.unwrap(),
        1,
        "Expected 1 user, but found {}.",
        user_count.count.unwrap()
    );
}




#[rocket::async_test]
async fn test_register_passwords_do_not_match() {
    let pool = setup_test_db().await;

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![register]);

    let client = Client::tracked(rocket).await.unwrap();

    let response = client
        .post("/register")
        .header(rocket::http::ContentType::JSON)
        .body(json!({
            "username": "test_user",
            "password": "password123",
            "password2": "password456",
            "email": "test_user@example.com"
        }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_json::<RegisterResponse>().await.unwrap();
    assert!(!body.success);
    assert_eq!(body.message, "Passwords do not match");
}

#[rocket::async_test]
async fn test_register_username_taken() {
    let pool = setup_test_db().await;

    // Insert a user with the same username
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "test_user",
        "existing_user@example.com",
        "hashed_password"
    )
    .execute(&pool)
    .await
    .expect("Failed to insert existing user");

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![register]);

    let client = Client::tracked(rocket).await.unwrap();

    let response = client
        .post("/register")
        .header(rocket::http::ContentType::JSON)
        .body(json!({
            "username": "test_user",
            "password": "password123",
            "password2": "password123",
            "email": "new_user@example.com"
        }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_json::<RegisterResponse>().await.unwrap();
    assert!(!body.success);
    assert_eq!(body.message, "Username already taken");
}

#[rocket::async_test]
async fn test_register_email_taken() {
    let pool = setup_test_db().await;

    // Insert a user with the same email
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        "existing_user",
        "test_user@example.com",
        "hashed_password"
    )
    .execute(&pool)
    .await
    .expect("Failed to insert existing user");

    let rocket = rocket::build()
        .manage(pool.clone())
        .mount("/", routes![register]);

    let client = Client::tracked(rocket).await.unwrap();

    let response = client
        .post("/register")
        .header(rocket::http::ContentType::JSON)
        .body(json!({
            "username": "new_user",
            "password": "password123",
            "password2": "password123",
            "email": "test_user@example.com"
        }).to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body = response.into_json::<RegisterResponse>().await.unwrap();
    assert!(!body.success);
    assert_eq!(body.message, "Email already taken");
}