mod alerts;
mod card;
mod checklist;
mod compare;
mod daylight;
mod gpx;
mod hunt;
mod list;
mod log;
mod nearby;
mod park;
mod random;
mod share;
mod stats;
mod streak;
mod trail;
mod weather;

pub use alerts::handle_alerts;
pub use card::print_card;
pub use checklist::handle_checklist;
pub use compare::handle_compare;
pub use daylight::handle_daylight;
pub use gpx::export_gpx;
pub use hunt::handle_hunt;
pub use list::handle_list;
pub use log::handle_log;
pub use nearby::handle_nearby;
pub use park::handle_park;
pub use random::handle_random;
pub use share::handle_share;
pub use stats::handle_stats;
pub use streak::handle_streak;
pub use trail::handle_trail;
pub use weather::handle_weather;

use crate::conditions::{format_condition_url, get_park_url};
use crate::icons::Icons;
use crate::services::weather::get_weather;
use crate::trails::{Difficulty, Trail};
use anyhow::Result;
use colored::{ColoredString, Colorize};

/// Shared helper function for printing trail information
pub fn print_trail_info(trail: &Trail, show_weather: bool) -> Result<()> {
    println!("\n  {}", trail.name.bold());

    let difficulty_display = match trail.difficulty {
        Some(diff) => format_difficulty(diff),
        None => "Non spécifié".normal(),
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

fn format_difficulty(difficulty: Difficulty) -> ColoredString {
    match difficulty {
        Difficulty::Facile => "Facile".green(),
        Difficulty::Intermediaire => "Intermédiaire".yellow(),
        Difficulty::Difficile => "Difficile".red(),
    }
}
