// src/main.rs

#[macro_use]
extern crate rocket;

pub mod api;
pub mod db;
pub mod routes;
pub mod security;

use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::env;

#[launch]
async fn rocket() -> _ {
    // Load environment variables from .env file (optional)
    dotenv().ok();

    // Create the database pool
    let pool = db::pool::get_pool(&env::var("DATABASE_URL").unwrap()).await;

    // Build and return the Rocket instance
    rocket::build()
        .attach(Template::fairing())
        .manage(pool)
        .mount(
            "/static",
            FileServer::from(env::var("STATIC_PATH").unwrap()),
        )
        .mount(
            "/",
            routes![
                routes::pages::index,
                routes::pages::about,
                routes::pages::login,
                routes::pages::register,
                routes::pages::search,
            ],
        )
        .mount("/api", routes![api::login::login, api::register::register])
}
