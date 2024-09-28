use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::postgres::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: Option<String>,
}

#[get("/search?<q>&<language>")]  
pub async fn search(
    q: Option<String>,          
    language: Option<String>,   
    db: &State<PgPool>
) -> Json<Vec<SearchResult>> {
    // Default to English if no language is specified
    let language = language.unwrap_or_else(|| "en".to_string());

    // If the search query is empty, return an empty JSON array
    let query_string = match q {
        Some(query) if !query.is_empty() => format!("%{}%", query),
        _ => return Json(vec![]),  
    };

    // Query the database for pages that match the search query
    let search_results: Vec<SearchResult> = sqlx::query_as::<_, SearchResult>(
        "SELECT title, url, content FROM pages WHERE language = $1 AND content LIKE $2",
    )
    .bind(&language)
    .bind(&query_string)
    .fetch_all(db.inner())
    .await
    .unwrap_or_else(|err| {
        eprintln!("Database query error: {:?}", err);
        vec![]
    });

    // Return the search results as JSON
    Json(search_results)
}
