use crate::icons::Icons;
use crate::services::weather::{get_7day_forecast, get_weather};
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;
use chrono::Datelike;
use colored::Colorize;

pub fn handle_weather(trail_name: &str, week: bool) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;

    if week {
        handle_week_forecast(trail)?;
    } else {
        let weather = get_weather(trail.lat, trail.lng)?;
        println!(
            "\n{} {}",
            Icons::weather(weather.weather_code),
            trail.name.bold()
        );
        println!("  Temperature: {:.0}°C", weather.temperature);
        println!("  Wind: {:.0}km/h", weather.wind_speed);
        println!("  Conditions: {}", weather.description());
    }

    Ok(())
}

fn handle_week_forecast(trail: &crate::trails::Trail) -> Result<()> {
    let forecast = get_7day_forecast(trail.lat, trail.lng)?;

    println!("\n{}  7-Day Forecast for {} ({})\n", Icons::WEATHER, trail.name.bold(), trail.park);

    let mut best_days = Vec::new();

    for day in forecast.iter() {
        let (rating_icon, rating_text) = day.rating();
        let weekday = day.date.weekday();
        let day_name = match weekday {
            chrono::Weekday::Mon => "Mon",
            chrono::Weekday::Tue => "Tue",
            chrono::Weekday::Wed => "Wed",
            chrono::Weekday::Thu => "Thu",
            chrono::Weekday::Fri => "Fri",
            chrono::Weekday::Sat => "Sat",
            chrono::Weekday::Sun => "Sun",
        };

        let day_num = day.date.day();
        println!(
            "  {} {:02}   {}  {:.0}°C  Wind: {:.0}km/h  {} {}",
            day_name,
            day_num,
            day.icon(),
            day.max_temp,
            day.wind_speed,
            rating_icon,
            rating_text
        );

        if rating_text == "Excellent" || rating_text == "Good" {
            best_days.push((day_name, day_num));
        }
    }

    if !best_days.is_empty() {
        println!("\n  {} Best days: {}",
            Icons::SUCCESS, 
            best_days.iter()
                .map(|(name, num)| format!("{}{:02}", name, num))
                .collect::<Vec<_>>()
                .join(", "));
    }

    Ok(())
}

