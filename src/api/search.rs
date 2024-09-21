#[macro_use] extern create rocket;

use rocket_contrib::templates::Template;
use sqlx::postgres::PgPool;
use rocket::State;
use std::sync::Arc;
use rocket_contrib::json::Json;
use std::collections::HashMap;

mod db;

#[derive(Serialize)]
struct SearchResult {
    title: String,
    url: String,
    description: String,
}
