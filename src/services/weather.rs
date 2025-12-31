use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    weathercode: u32,
}

#[derive(Debug, Clone)]
pub struct Weather {
    pub temperature: f64,
    pub wind_speed: f64,
    pub weather_code: u32,
}

impl Weather {
    pub fn description(&self) -> &'static str {
        match self.weather_code {
            0 => "clear sky",
            1..=3 => "partly cloudy",
            45 | 48 => "foggy",
            51 | 53 | 55 => "drizzle",
            56 | 57 => "freezing drizzle",
            61 | 63 | 65 => "rain",
            66 | 67 => "freezing rain",
            71 | 73 | 75 => "snow",
            77 => "snow grains",
            80..=82 => "rain showers",
            85 | 86 => "snow showers",
            95 => "thunderstorm",
            96 | 99 => "thunderstorm with hail",
            _ => "unknown",
        }
    }
}

pub fn get_weather(lat: f64, lng: f64) -> Result<Weather> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        lat, lng
    );

    let weather_resp: WeatherResponse = reqwest::blocking::get(&url)
        .context("Failed to fetch weather data")?
        .json()
        .context("Failed to parse weather response")?;

    Ok(Weather {
        temperature: weather_resp.current_weather.temperature,
        wind_speed: weather_resp.current_weather.windspeed,
        weather_code: weather_resp.current_weather.weathercode,
    })
}
