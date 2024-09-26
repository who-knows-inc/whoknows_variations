use reqwest::Client; // using reqwest crate to make HTTP requests
use serde::{Deserialize, Serialize}; // using serde to serialize and deserialize JSON

// Define the structure of the JSON response from the weather API
#[derive(Deserialize, Serialize)] // derive the Deserialize and Serialize traits
pub struct WeatherData { // define a struct named WeatherData
    current: CurrentWeather, // define a field named current of type CurrentWeather
}

// Define the structure of the current weather data
#[derive(Deserialize, Serialize)] // derive the Deserialize and Serialize traits
pub struct CurrentWeather { // define a struct named CurrentWeather
    temperature_2m: f32, // define a field named temperature_2m of type f32
    relative_humidity_2m: f32, // define a field named relative_humidity_2m of type f32
    rain: f32, // define a field named rain of type f32
    wind_speed_10m: f32, // define a field named wind_speed_10m of type f32
    wind_direction_10m: f32, // define a field named wind_direction_10m of type f32
}

// Define a function to fetch weather data from the API
pub async fn fetch_weather_data() -> CurrentWeather { // define an asynchronous function named fetch_weather_data that returns a CurrentWeather struct
    let client = Client::new(); // create a new reqwest Client so we can make HTTP requests
    // define the URL of the weather API
    let api_url = "https://api.open-meteo.com/v1/forecast?latitude=55.6759&longitude=12.5655&current=temperature_2m,relative_humidity_2m,rain,wind_speed_10m,wind_direction_10m&timezone=auto";
    
    // make a GET request to the weather API and wait for the response
    let response = client.get(api_url).send().await.unwrap(); // send the request and unwrap the response
    let weather_data: WeatherData = response.json().await.unwrap(); // parse the JSON response into a WeatherData struct and unwrap it
    
    weather_data.current // return the current weather data
}
