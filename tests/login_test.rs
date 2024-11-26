use rocket::{http::{ContentType, Status}, local::asynchronous::Client};
use serde_json::json;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use whoknows_nooneknows::routes::api::login::{login, LoginResponse};

/// Returnerer en mock SQLite-databaseforbindelse
async fn setup_mock_db() -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to SQLite");

    // Opret tabel til testen
    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );

        INSERT INTO users (username, password)
        VALUES
            ('test_user', 'correct_password'),
            ('another_user', 'hashed_password');
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to set up mock database");

    pool
}

async fn setup_rocket_with_mock_db(mock_db: Pool<Sqlite>) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(mock_db) // Tilføj mock database som en delt tilstand
        .mount("/", rocket::routes![login]) // Mount login-endpoint
}

#[rocket::async_test]
async fn test_successful_login() {
    let mock_db = setup_mock_db().await; // Opret mock database
    let rocket = setup_rocket_with_mock_db(mock_db).await; // Start Rocket med mock database
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let login_request = json!({
        "username": "test_user",
        "password": "correct_password"
    });

    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .body(login_request.to_string())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    let body: LoginResponse = response.into_json().await.unwrap();
    assert!(body.success);
    assert_eq!(body.message, "Login successful");
}
