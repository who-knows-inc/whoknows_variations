use rocket::fairing::{Fairing, Info, Kind};
use rocket::outcome::Outcome;
use rocket::{Data, Request, State};
use sqlx::PgPool;

pub struct DbConnectionChecker;

#[rocket::async_trait]
impl Fairing for DbConnectionChecker {
    fn info(&self) -> Info {
        Info {
            name: "Database Connection Checker",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // Get the database pool from the request's managed state
        println!("Checking database connection...");
        // Print out the request path
        println!("Request path: {:?}", request.uri());
        println!("Request method: {:?}", request.method());
        println!("Request headers: {:?}", request.headers());
        if let Outcome::Success(pool) = request.guard::<&State<PgPool>>().await {
            if let Err(e) = pool.acquire().await {
                eprintln!("Failed to acquire database connection: {:?}", e);
            }
        } else {
            eprintln!("Database pool not available in the request state");
            // Handle the error if needed
        }
    }
}
