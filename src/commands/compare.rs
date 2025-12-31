use crate::icons::Icons;
use crate::services::weather::get_weather;
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;
use colored::Colorize;

pub fn handle_compare(trail1_name: &str, trail2_name: &str) -> Result<()> {
    let trails = load_trails()?;

    let trail1 = find_trail_by_name(&trails, trail1_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail1_name))?;
    let trail2 = find_trail_by_name(&trails, trail2_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail2_name))?;

    let weather1 = get_weather(trail1.lat, trail1.lng).ok();
    let weather2 = get_weather(trail2.lat, trail2.lng).ok();

    println!("\n┌─────────────────────┬─────────────────────┐");
    println!(
        "│ {:19} │ {:19} │",
        trail1.name.to_uppercase(),
        trail2.name.to_uppercase()
    );
    println!("├─────────────────────┼─────────────────────┤");
    println!("│ {:19} │ {:19} │", trail1.park, trail2.park);
    println!(
        "│ {:19} │ {:19} │",
        format_difficulty(&trail1.difficulty),
        format_difficulty(&trail2.difficulty)
    );
    println!(
        "│ {:19} │ {:19} │",
        format!("{:.1} km", trail1.length_km),
        format!("{:.1} km", trail2.length_km)
    );
    let hours1 = (trail1.length_km / 3.0).ceil() as u32;
    let hours2 = (trail2.length_km / 3.0).ceil() as u32;
    println!(
        "│ {:19} │ {:19} │",
        format!("~{}-{}h", hours1.saturating_sub(1).max(1), hours1 + 1),
        format!("~{}-{}h", hours2.saturating_sub(1).max(1), hours2 + 1)
    );
    println!(
        "│ {:19} │ {:19} │",
        format!("{:.0}km from Montreal", trail1.distance_from_mtl),
        format!("{:.0}km from Montreal", trail2.distance_from_mtl)
    );
    println!("├─────────────────────┼─────────────────────┤");

    let weather_line1 = if let Some(w) = &weather1 {
        format!(
            "Today: {} {:.0}°C",
            Icons::weather(w.weather_code),
            w.temperature
        )
    } else {
        "Weather unavailable".to_string()
    };

    let weather_line2 = if let Some(w) = &weather2 {
        format!(
            "Today: {} {:.0}°C",
            Icons::weather(w.weather_code),
            w.temperature
        )
    } else {
        "Weather unavailable".to_string()
    };

    println!("│ {:19} │ {:19} │", weather_line1, weather_line2);
    println!("└─────────────────────┴─────────────────────┘");

    Ok(())
}

fn format_difficulty(difficulty: &str) -> String {
    match difficulty.to_lowercase().as_str() {
        "facile" => difficulty.green().to_string(),
        "intermédiaire" | "intermediaire" => difficulty.yellow().to_string(),
        "difficile" => difficulty.red().to_string(),
        _ => difficulty.to_string(),
    }
}
