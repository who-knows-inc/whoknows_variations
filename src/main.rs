#[macro_use]
extern crate rocket;

pub mod db;
pub mod routes;

use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::env;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let static_path = env::var("STATIC_PATH").unwrap_or("/var/www/whoknows/static".to_string());

    let pool = db::pool::get_pool().await;

    rocket::build()
        .attach(Template::fairing())
        .manage(pool)
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
        .mount("/static", FileServer::from(static_path))
}
