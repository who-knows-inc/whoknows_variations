use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn get_pool(database_url: &str) -> PgPool {
    // Connect to the database using the URL provided
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations
    MIGRATOR.run(&pool).await.expect("Failed to run migrations");

    // Return the pool
    pool
}
