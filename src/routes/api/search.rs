use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize, Debug)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

#[get("/search?<language>&<q>")]
pub async fn search(
    language: String,
    q: Option<String>,
    pool: &State<PgPool>,
) -> Json<Vec<SearchResult>> {
    let language = language.to_lowercase();
    let query_string = match &q {
        Some(q) if !q.is_empty() => format!("%{}%", q.to_lowercase()),
        _ => return Json(vec![]),
    };

    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to acquire connection: {:?}", e);
            return Json(vec![]);
        }
    };
    let search_results = sqlx::query_as!(
        SearchResult,
        "SELECT title, url, content FROM pages WHERE language = $1 AND content LIKE $2",
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
    Json(search_results)
}
