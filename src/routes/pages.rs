use rocket::{http::CookieJar, State};
use rocket_dyn_templates::{context, Template};
use sqlx::PgPool;
use rocket::get;

use crate::db::get_current_user::get_current_user;
use crate::routes::api::weather::fetch_weather_data;

#[get("/")]
pub async fn index(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    let user = get_current_user(cookies, db_pool).await;
    Template::render("search", context! { user: user })
}

#[get("/about")]
pub async fn about(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    let user = get_current_user(cookies, db_pool).await;
    Template::render("about", context! { user: user })
}

#[get("/login")]
pub async fn login(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    if let Some(user) = get_current_user(cookies, db_pool).await {
        Template::render("/", context! { user: user })
    } else {
        Template::render("login", context! {})
    }
}

#[get("/register")]
pub async fn register(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    if let Some(user) = get_current_user(cookies, db_pool).await {
        Template::render("/", context! { user: user })
    } else {
        Template::render("register", context! {})
    }
}

#[get("/weather")] // add this route to the routes module
                   // we need to add the fetch_weather_data function to the weather module
pub async fn weather(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    // get the current user
    let user = get_current_user(cookies, db_pool).await;
    // fetch the weather data
    let weather_data = fetch_weather_data().await.into_inner(); // calls the fetch_weather_data function to get the current weather data
                                                                // render the weather template with the user and weather data
    Template::render("weather", context! { user: user, weather: weather_data })
}
