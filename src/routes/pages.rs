use rocket::{http::CookieJar, serde::json::Json, State};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;
use sqlx::PgPool;

use crate::db::get_current_user::get_current_user;

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

#[get("/search")]
pub async fn search(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    let user = get_current_user(cookies, db_pool).await;
    Template::render("search", context! { user: user })
}

// THIS IS THE WEATHER API DUMMY JUST RETURNS SOME DATA
//  TODO - REPLACE THIS WITH A REAL API CALL

type WeatherResponse = WeatherResponseData;

#[derive(Serialize)]
pub struct WeatherResponseData {
    pub success: bool,
    pub message: String,
    pub weather: Weather,
}

#[derive(Serialize)]
pub struct Weather {
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
}

// weather api
#[get("/weather")]
pub async fn weather() -> Json<WeatherResponse> {
    let weather_response = WeatherResponse {
        success: true,
        message: "Weather data retrieved successfully".to_string(),
        weather: Weather {
            temperature: 25.0,
            humidity: 50.0,
            wind_speed: 10.0,
        },
    };
    Json(weather_response)
}
