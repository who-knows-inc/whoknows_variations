use rocket::{http::Status, local::asynchronous::Client, routes};
use whoknows_nooneknows::routes::api::weather::fetch_weather_data;

#[rocket::async_test]
async fn test_weather_endpoint() {
    let rocket = rocket::build().mount("/", routes![fetch_weather_data]);
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    let response = client.get("/weather").dispatch().await;

    assert_eq!(response.status(), Status::Ok);

    // Dummy assertion for now
    let body = response.into_string().await.unwrap();
    assert!(body.contains("temperature_2m"));
}
