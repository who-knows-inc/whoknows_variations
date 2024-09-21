use dotenvy::dotenv;
use std::env;

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn get_pool() -> PgPool {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database using the URL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations
    MIGRATOR.run(&pool).await.expect("Failed to run migrations");

    // Return the pool
    pool
}
