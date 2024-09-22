use rocket::{http::CookieJar, State};
use rocket_dyn_templates::{context, Template};
use sqlx::PgPool;

use crate::db::get_current_user::get_current_user;

#[get("/")]
pub async fn index(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    let user = get_current_user(cookies, db_pool).await;
    Template::render("index", context! { user: user })
}

#[get("/about")]
pub async fn about(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    let user = get_current_user(cookies, db_pool).await;
    Template::render("about", context! { user: user })
}

#[get("/login")]
pub async fn login(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    if let Some(user) = get_current_user(&cookies, &db_pool).await {
        Template::render("/", context! { user: user })
    } else {
        Template::render("login", context! {})
    }
}

#[get("/register")]
pub async fn register(cookies: &CookieJar<'_>, db_pool: &State<PgPool>) -> Template {
    if let Some(user) = get_current_user(&cookies, &db_pool).await {
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
