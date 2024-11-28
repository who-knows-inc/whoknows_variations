#[cfg(test)]
mod tests {
    use rocket::{local::asynchronous::Client, routes};
    use sqlx::{PgPool, Executor};
    use rocket::http::Status;
    use whoknows_nooneknows::routes::api::search::{search, SearchResult};

    // Set up a test database
    async fn setup_test_db() -> PgPool {
        let db_url = "postgres://postgres:postgres@localhost:5432/postgres"; // Replace with your actual database URL
        let pool = PgPool::connect(db_url)
            .await
            .expect("Failed to connect to the database");

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS pages (
                id SERIAL PRIMARY KEY,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                language TEXT NOT NULL CHECK (language IN ('en', 'fr')),
                content TEXT
            );
            TRUNCATE TABLE pages;
            INSERT INTO pages (title, url, language, content)
            VALUES
            ('Rust Documentation', 'https://doc.rust-lang.org/', 'en', 'Rust is a programming language used for systems programming.'),
            ('Rust Langue Française', 'https://doc.rust-lang.org/fr', 'fr', 'Rust est un langage de programmation.');
            "#
        )
        .await
        .expect("Failed to set up test database");

        pool
    }

    #[rocket::async_test]
    async fn test_search_with_results() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![search]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .get("/search?q=Rust&language=en")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<Vec<SearchResult>>().await.unwrap();
        assert!(!body.is_empty(), "Expected results, but got an empty response.");
        assert_eq!(body[0].title, "Rust Documentation");
    }

    #[rocket::async_test]
    async fn test_search_no_results() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![search]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .get("/search?q=NonExistent&language=en")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<Vec<SearchResult>>().await.unwrap();
        assert!(body.is_empty(), "Expected no results, but got some.");
    }

    #[rocket::async_test]
    async fn test_search_with_no_query() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![search]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .get("/search?language=en")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::BadRequest);
    }

    #[rocket::async_test]
    async fn test_search_with_different_language() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![search]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .get("/search?q=Rust&language=fr")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<Vec<SearchResult>>().await.unwrap();
        assert!(!body.is_empty(), "Expected results for 'language=fr', but got an empty response.");
        assert_eq!(body[0].title, "Rust Langue Française");
    }
}