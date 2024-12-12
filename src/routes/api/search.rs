use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use rocket::State;
use rocket::get;




#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

#[get("/search?<q>&<language>")]
pub async fn search(
    language: Option<String>,
    q: Option<String>,
    pool: &State<PgPool>,
) -> Result<Json<Vec<SearchResult>>, Custom<Json<&'static str>>> {
    let language = language.unwrap_or_else(|| "en".to_string());
    let query_string = match &q {
        Some(q) if !q.is_empty() => format!("%{}%", q.to_lowercase()),
        _ => return Err(Custom(Status::BadRequest, Json("Query parameter 'q' is required"))),
    };

    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire connection: {:?}", e);
            return Err(Custom(Status::InternalServerError, Json("Failed to acquire connection")));
        }
    };
    let search_results = sqlx::query_as!(
        SearchResult,
        "SELECT title, url FROM pages WHERE language = $1 AND (title ILIKE $2 OR content ILIKE $2)",
        language,
        query_string
    )
    .fetch_all(&mut conn)
    .await
    .unwrap_or_else(|err| {
        eprintln!("Database query error: {:?}", err);
        vec![] // Return an empty vector if there is an error
    });

    // Return the search results as JSON
    Ok(Json(search_results))
}