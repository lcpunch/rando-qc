use anyhow::{Context, Result};
use chrono::NaiveDate;
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

#[derive(Debug, Deserialize)]
struct ForecastResponse {
    daily: DailyForecast,
}

#[derive(Debug, Deserialize)]
struct DailyForecast {
    time: Vec<String>,
    temperature_2m_max: Vec<f64>,
    precipitation_sum: Vec<f64>,
    windspeed_10m_max: Vec<f64>,
    weathercode: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Weather {
    pub temperature: f64,
    pub wind_speed: f64,
    pub weather_code: u32,
}

#[derive(Debug, Clone)]
pub struct DailyWeather {
    pub date: NaiveDate,
    pub max_temp: f64,
    pub precipitation: f64,
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

impl DailyWeather {
    pub fn rating(&self) -> (&'static str, &'static str) {
        let has_rain = self.precipitation > 0.5;
        let high_wind = self.wind_speed > 20.0;
        let storm = self.weather_code >= 95;

        if storm || (has_rain && high_wind) {
            ("[!]", "Bad")
        } else if has_rain || high_wind {
            ("[!]", "Okay")
        } else if self.weather_code <= 3 && self.wind_speed < 15.0 {
            ("[OK]", "Excellent")
        } else {
            ("[OK]", "Good")
        }
    }

    pub fn icon(&self) -> &'static str {
        crate::icons::Icons::weather(self.weather_code)
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

pub fn get_7day_forecast(lat: f64, lng: f64) -> Result<Vec<DailyWeather>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,precipitation_sum,windspeed_10m_max,weathercode&timezone=America/Toronto",
        lat, lng
    );

    let forecast_resp: ForecastResponse = reqwest::blocking::get(&url)
        .context("Failed to fetch forecast data")?
        .json()
        .context("Failed to parse forecast response")?;

    let daily = &forecast_resp.daily;
    daily
        .time
        .iter()
        .zip(&daily.temperature_2m_max)
        .zip(&daily.precipitation_sum)
        .zip(&daily.windspeed_10m_max)
        .zip(&daily.weathercode)
        .map(
            |((((time, &max_temp), &precipitation), &wind_speed), &weather_code)| {
                let date =
                    NaiveDate::parse_from_str(time, "%Y-%m-%d").context("Failed to parse date")?;
                Ok(DailyWeather {
                    date,
                    max_temp,
                    precipitation,
                    wind_speed,
                    weather_code,
                })
            },
        )
        .collect()
}
