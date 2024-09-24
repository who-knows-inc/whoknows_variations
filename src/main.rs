// src/main.rs

#[macro_use]
extern crate rocket;

pub mod db;
pub mod fairings;
pub mod models;
pub mod routes;
pub mod security;

use db::pool::get_pool;
use dotenvy::dotenv;
use fairings::connection_checker::DbConnectionChecker;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::env;

#[launch]
async fn rocket() -> _ {
    // Load environment variables from .env file (optional)
    dotenv().ok();

    let figment = rocket::Config::figment().merge((
        "root",
        env::var("ROCKET_BASE").expect("ROCKET_BASE must be set"),
    ));

    // Create the database pool
    let pool = get_pool(&env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await;

    // Build and return the Rocket instance
    rocket::custom(figment)
        .attach(Template::fairing())
        .attach(DbConnectionChecker)
        .manage(pool)
        .mount(
            "/static",
            FileServer::from(env::var("STATIC_PATH").expect("STATIC_PATH must be set")),
        )
        .mount(
            "/",
            routes![
                routes::pages::index,
                routes::pages::about,
                routes::pages::login,
                routes::pages::register,
                routes::pages::search,
                routes::pages::weather,
            ],
        )
        .mount(
            "/api",
            routes![
                routes::api::login::login,
                routes::api::login::logout,
                routes::api::register::register,
                routes::api::weather::weather,
                routes::api::search::search,
            ],
        )
}
