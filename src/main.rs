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
    dotenv().ok();

    let pool = db::pool::get_pool().await;

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
                routes::pages::search
            ],
        )
        .mount("/api", routes![api::login::login, api::register::register])
}
