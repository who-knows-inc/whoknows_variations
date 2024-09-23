
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WeatherData {
    current: CurrentWeather,
}

#[derive(Deserialize, Serialize)]
pub struct CurrentWeather {
    temperature_2m: f32,
    relative_humidity_2m: f32,
    rain: f32,
    wind_speed_10m: f32,
    wind_direction_10m: f32,
}

pub async fn fetch_weather_data() -> CurrentWeather {
    let client = Client::new();
    let api_url = "https://api.open-meteo.com/v1/forecast?latitude=55.6759&longitude=12.5655&current=temperature_2m,relative_humidity_2m,rain,wind_speed_10m,wind_direction_10m&timezone=auto";
    
    let response = client.get(api_url).send().await.unwrap();
    let weather_data: WeatherData = response.json().await.unwrap();
    
    weather_data.current
}
