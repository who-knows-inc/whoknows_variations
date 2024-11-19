use serde::Serialize;
use sqlx::FromRow;
use time::PrimitiveDateTime;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: PrimitiveDateTime,
}
