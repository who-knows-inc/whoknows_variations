// THIS IS THE WEATHER API DUMMY JUST RETURNS SOME DATA
//  TODO - REPLACE THIS WITH A REAL API CALL

use rocket::serde::json::Json;
use serde::Serialize;

type WeatherResponse = WeatherResponseData;

#[derive(Serialize)]
pub struct WeatherResponseData {
    pub success: bool,
    pub message: String,
    pub weather: Weather,
}

#[derive(Serialize)]
pub struct Weather {
    pub temperature: f64,
    pub humidity: f64,
    pub wind_speed: f64,
}

// weather api
#[get("/weather")]
pub async fn weather() -> Json<WeatherResponse> {
    let weather_response = WeatherResponse {
        success: true,
        message: "Weather data retrieved successfully".to_string(),
        weather: Weather {
            temperature: 25.0,
            humidity: 50.0,
            wind_speed: 10.0,
        },
    };
    Json(weather_response)
}
