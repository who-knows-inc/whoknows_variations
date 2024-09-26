use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sqlx::postgres::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(FromForm)]
pub struct SearchData {
    pub q: Option<String>,
    pub language: Option<String>,
}

#[post("/search", data = "<search_data>")]
pub async fn search(search_data: Form<SearchData>, db: &State<PgPool>) -> Json<Vec<SearchResult>> {
    let language = search_data
        .language
        .clone()
        .unwrap_or_else(|| "en".to_string());
    let query_string = match &search_data.q {
        Some(q) if !q.is_empty() => format!("%{}%", q),
        _ => return Json(vec![]),
    };

    let search_results: Vec<SearchResult> = sqlx::query_as::<_, SearchResult>(
        "SELECT title, url, description FROM pages WHERE language = $1 AND content LIKE $2",
    )
    .bind(&language)
    .bind(&query_string)
    .fetch_all(db.inner())
    .await
    .unwrap_or_else(|err| {
        eprintln!("Database query error: {:?}", err);
        vec![]
    });

    Json(search_results)
}
