use crate::models::user::User;
use rocket::http::{Cookie, CookieJar};
use sqlx::PgPool;

pub async fn get_current_user(cookies: &CookieJar<'_>, db_pool: &PgPool) -> Option<User> {
    // Print the cookies
    println!("Cookies: {:?}", cookies.get("user_id"));
    // Retrieve the private cookie named "user_id"
    if let Some(cookie) = cookies.get("user_id") {
        // Parse the user ID from the cookie
        if let Ok(user_id) = cookie.value().parse::<i32>() {
            // Query the database for the user
            match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
                .fetch_one(db_pool)
                .await
            {
                Ok(user) => Some(user),
                Err(sqlx::Error::RowNotFound) => {
                    // User not found; remove the invalid cookie
                    cookies.remove_private(Cookie::from("user_id"));
                    None
                }
                Err(_) => None, // Handle other errors as needed
            }
        } else {
            // Invalid user ID in cookie; remove it
            cookies.remove_private(Cookie::from("user_id"));
            None
        }
    } else {
        // No user ID cookie found
        None
    }
}
