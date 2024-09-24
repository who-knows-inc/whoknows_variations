use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use serde::Serialize;
use sqlx::postgres::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: Option<String>,
}

#[derive(FromForm)]  
pub struct SearchData {
    pub q: Option<String>,
    pub language: Option<String>,
}

#[rocket::get("/search?<q>&<language>")]
pub async fn search(
    q: Option<String>,  
    language: Option<String>,
    db: &State<PgPool>
) -> Result<Json<Vec<SearchResult>>, Status> {
    let language = language.unwrap_or_else(|| "en".to_string());
    let query_string = match q {
        Some(ref q) if !q.is_empty() => format!("%{}%", q),
        _ => return Err(Status::BadRequest),  // Returnerer 400 Bad Request, hvis der ikke er en søgeord
    };

    eprintln!("Searching with language: {} and query: {}", language, query_string);  // Logger søgeparametre

    let search_results: Vec<SearchResult> = sqlx::query_as::<_, SearchResult>(
        "SELECT title, url, content FROM pages WHERE language = $1 AND content ILIKE $2"
    )
    .bind(&language)
    .bind(&query_string)
    .fetch_all(db.inner())
    .await
    .map_err(|err| {
        eprintln!("Database query error: {:?}", err);
        Status::InternalServerError
    })?;

    Ok(Json(search_results))
}
