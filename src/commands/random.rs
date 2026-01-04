use crate::icons::Icons;
use crate::services::weather::get_weather;
use crate::trails::{Difficulty, filter_trails, load_trails};
use anyhow::Result;
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn handle_random(difficulty: Option<String>, max_distance: Option<f64>) -> Result<()> {
    let trails = load_trails()?;

    let difficulty_filter = difficulty.and_then(|d| d.parse().ok());
    let filtered = filter_trails(&trails, difficulty_filter, max_distance, None, None, None);

    if filtered.is_empty() {
        println!("{}", "No trails found matching your criteria.".yellow());
        return Ok(());
    }

    let mut rng = thread_rng();
    let trail = filtered
        .choose(&mut rng)
        .ok_or_else(|| anyhow::anyhow!("No trail selected"))?;

    println!("\n{} Random pick:\n", Icons::RANDOM);
    println!("  {}", trail.name.bold());
    println!("  {}", trail.park);
    println!(
        "  {} • {:.1}km • ~{:.0}h",
        format_difficulty(trail.difficulty),
        trail.length_km,
        (trail.length_km / 3.0).ceil()
    );
    println!("  {:.0}km from Montreal", trail.distance_from_mtl);

    if let Ok(weather) = get_weather(trail.lat, trail.lng) {
        println!(
            "\n  Today: {} {:.0}°C, {}",
            Icons::weather(weather.weather_code),
            weather.temperature,
            weather.description()
        );
    }

    if !trail.park_code.is_empty() {
        println!(
            "\n  → Check conditions: sepaq.com/pq/{}/",
            trail.park_code.to_lowercase()
        );
    }

    Ok(())
}

fn format_difficulty(difficulty: Option<Difficulty>) -> colored::ColoredString {
    match difficulty {
        Some(Difficulty::Facile) => "Facile".green(),
        Some(Difficulty::Intermediaire) => "Intermédiaire".yellow(),
        Some(Difficulty::Difficile) => "Difficile".red(),
        None => "Non spécifié".normal(),
    }
}
