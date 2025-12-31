use crate::conditions::{format_condition_url, get_park_url};
use crate::icons::Icons;
use crate::services::weather::get_weather;
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;
use colored::{ColoredString, Colorize};

pub fn handle_trail(trail_name: &str) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;

    println!("\n{} {}", Icons::TRAIL.green(), trail.name.bold());
    println!("  Park: {}", trail.park);

    let difficulty_display =
        if trail.difficulty.trim().is_empty() || trail.difficulty.eq_ignore_ascii_case("Unknown") {
            "Non spécifié".normal()
        } else {
            format_difficulty(&trail.difficulty)
        };

    println!("  Difficulty: {}", difficulty_display);
    println!("  Length: {:.1}km", trail.length_km);
    println!("  Distance from Montreal: {:.0}km", trail.distance_from_mtl);

    match get_weather(trail.lat, trail.lng) {
        Ok(weather) => {
            println!(
                "  {} {:.0}°C, {}, wind {:.0}km/h",
                Icons::weather(weather.weather_code),
                weather.temperature,
                weather.description(),
                weather.wind_speed
            );
        }
        Err(e) => {
            println!("  {} Weather unavailable: {}", Icons::WARNING.yellow(), e);
        }
    }

    if !trail.park_code.is_empty() {
        println!(
            "  {}",
            format_condition_url(&get_park_url(&trail.park_code), Icons::LINK)
        );
    }

    Ok(())
}

fn format_difficulty(difficulty: &str) -> ColoredString {
    match difficulty.to_lowercase().as_str() {
        "facile" => difficulty.green(),
        "intermédiaire" | "intermediaire" => difficulty.yellow(),
        "difficile" => difficulty.red(),
        _ => difficulty.normal(),
    }
}
