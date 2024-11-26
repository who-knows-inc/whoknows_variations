use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::PgPool;
use rocket::get;
use rocket::serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}




#[get("/search?<q>&<language>")]
pub async fn search(
    language: Option<String>,
    q: Option<String>,
    pool: &State<PgPool>,
) -> Json<Vec<SearchResult>> {
    let language = language.unwrap_or_else(|| "en".to_string());
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
        "SELECT title, url FROM pages WHERE language = $1 AND content LIKE $2",
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
