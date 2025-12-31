mod card;
mod gpx;
mod list;
mod park;
mod trail;

pub use card::print_card;
pub use gpx::export_gpx;
pub use list::handle_list;
pub use park::handle_park;
pub use trail::handle_trail;

use crate::conditions::{format_condition_url, get_park_url};
use crate::icons::Icons;
use crate::services::weather::get_weather;
use crate::trails::Trail;
use anyhow::Result;
use colored::{ColoredString, Colorize};

/// Shared helper function for printing trail information
pub fn print_trail_info(trail: &Trail, show_weather: bool) -> Result<()> {
    println!("\n  {}", trail.name.bold());

    let difficulty_display =
        if trail.difficulty.trim().is_empty() || trail.difficulty.eq_ignore_ascii_case("Unknown") {
            "Non spécifié".normal()
        } else {
            format_difficulty(&trail.difficulty)
        };

    println!(
        "  {} • {:.1}km • {:.0}km from Montreal",
        difficulty_display, trail.length_km, trail.distance_from_mtl
    );

    if show_weather {
        if let Ok(weather) = get_weather(trail.lat, trail.lng) {
            println!(
                "  {} {:.0}°C, {}, wind {:.0}km/h",
                Icons::weather(weather.weather_code),
                weather.temperature,
                weather.description(),
                weather.wind_speed
            );
        }

        if !trail.park_code.is_empty() {
            println!(
                "  {}",
                format_condition_url(&get_park_url(&trail.park_code), Icons::LINK)
            );
        }
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
