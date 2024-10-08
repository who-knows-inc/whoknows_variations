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
    // Load environment variables from .env file
    dotenv().ok();

    // Create the database pool
    let pool = get_pool(&env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await;

    // Build and return the Rocket instance
    rocket::build()
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
                routes::pages::weather,
                routes::api::login::logout,
            ],
        )
        .mount(
            "/api",
            routes![
                routes::api::login::login,
                routes::api::register::register,
                routes::api::weather::fetch_weather_data,
                routes::api::search::search,
            ],
        )
}
