use crate::geo::distance_km;
use crate::icons::Icons;
use crate::trails::load_trails;
use anyhow::Result;
use colored::Colorize;

pub fn handle_nearby(
    lat: Option<f64>,
    lng: Option<f64>,
    park: Option<String>,
    radius: f64,
) -> Result<()> {
    let trails = load_trails()?;

    let (search_lat, search_lng) = if let (Some(lat), Some(lng)) = (lat, lng) {
        (lat, lng)
    } else if let Some(park_name) = park {
        // Find park center by averaging trail coordinates in that park
        let park_trails: Vec<_> = trails
            .iter()
            .filter(|t| t.park.to_lowercase().contains(&park_name.to_lowercase()))
            .collect();

        if park_trails.is_empty() {
            anyhow::bail!("Park not found: {}", park_name);
        }

        let avg_lat = park_trails.iter().map(|t| t.lat).sum::<f64>() / park_trails.len() as f64;
        let avg_lng = park_trails.iter().map(|t| t.lng).sum::<f64>() / park_trails.len() as f64;
        (avg_lat, avg_lng)
    } else {
        anyhow::bail!("Must provide either --lat/--lng or --park");
    };

    let mut nearby_trails: Vec<_> = trails
        .iter()
        .map(|trail| {
            let dist = distance_km(search_lat, search_lng, trail.lat, trail.lng);
            (trail, dist)
        })
        .filter(|(_, dist)| *dist <= radius)
        .collect();

    nearby_trails.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    if nearby_trails.is_empty() {
        println!(
            "{} No trails found within {}km of ({:.4}, {:.4})",
            Icons::LOCATION.yellow(),
            radius,
            search_lat,
            search_lng
        );
        return Ok(());
    }

    println!(
        "\n{} Trails within {:.0}km of ({:.4}, {:.4})\n",
        Icons::LOCATION.green(),
        radius,
        search_lat,
        search_lng
    );

    for (trail, dist) in nearby_trails {
        let difficulty_display = format_difficulty(&trail.difficulty);
        println!(
            "  {:.1}km   {} ({}) - {}, {:.1}km",
            dist,
            trail.name.bold(),
            trail.park,
            difficulty_display,
            trail.length_km
        );
    }

    Ok(())
}

fn format_difficulty(difficulty: &str) -> colored::ColoredString {
    match difficulty.to_lowercase().as_str() {
        "facile" => difficulty.green(),
        "intermédiaire" | "intermediaire" => difficulty.yellow(),
        "difficile" => difficulty.red(),
        _ => difficulty.normal(),
    }
}
