#[macro_use]
extern crate rocket;

pub mod api;
pub mod db;
pub mod routes;
pub mod security;

use std::env;

use dotenvy::dotenv;
// In src/main.rs
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    // Build Rocket instance without the pool
    let rocket = rocket::build()
        .attach(Template::fairing())
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
                routes::pages::search
            ],
        )
        .mount("/api", routes![api::login::login, api::register::register]);

    // Attach an ad-hoc fairing to initialize and manage the database pool
    rocket.attach(AdHoc::try_on_ignite("Database Pool", |rocket| async {
        // Extract the database URL from Rocket's configuration
        let database_url = rocket
            .figment()
            .extract_inner::<String>("databases.whoknows.url")
            .expect("database URL in Rocket.toml");

        // Create the database pool
        let pool = db::pool::get_pool(&database_url).await;

        // Attach the pool to Rocket's state
        Ok(rocket.manage(pool))
    }))
}
