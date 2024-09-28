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
    q: Option<String>,          // Extract the `q` query parameter
    language: Option<String>,   // Extract the `language` query parameter
    db: &State<PgPool>
) -> Json<Vec<SearchResult>> {
    // Use the specified language or default to "en"
    let language = language.unwrap_or_else(|| "en".to_string());

    // If there is a search query (q), format it as a SQL LIKE search
    let query_string = match q {
        Some(query) if !query.is_empty() => format!("%{}%", query),
        _ => return Json(vec![]),  
    };

    // Execute the SQL query
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
