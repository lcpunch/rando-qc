mod cache;
mod card;
mod cli;
mod conditions;
mod geo;
mod gpx;
mod icons;
mod services;
mod trails;
mod weather;

use crate::icons::Icons;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::{ColoredString, Colorize};
use conditions::{format_condition_url, get_park_url};
use trails::{filter_trails, find_trail_by_name, get_trails_by_park, load_trails};
use weather::get_weather;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update => {
            cache::download_trail_data()?;
            println!("{} Trail data updated successfully", Icons::SUCCESS.green());
        }
        Commands::List { .. } => handle_list(&cli)?,
        Commands::Park { name } => handle_park(name)?,
        Commands::Trail { name } => handle_trail(name)?,
        Commands::Card { name } => handle_card(name)?,
        Commands::Gpx { name, output } => handle_gpx(name, output.as_deref())?,
    }

    Ok(())
}

fn handle_list(cli: &Cli) -> Result<()> {
    if let Commands::List {
        difficulty: Some(ref diff_str),
        ..
    } = cli.command
        && trails::Difficulty::from_str(diff_str).is_none()
    {
        anyhow::bail!(
            "Invalid difficulty: '{}'. Valid options are: facile, intermediaire, difficile",
            diff_str
        );
    }

    let trails = load_trails()?;
    let filtered = filter_trails(
        &trails,
        cli.command.get_difficulty(),
        cli.command.get_max_distance(),
        cli.command.get_min_length(),
        cli.command.get_max_length(),
        cli.command.get_park_name(),
    );

    if filtered.is_empty() {
        println!("{}", "No trails found matching your criteria.".yellow());
        return Ok(());
    }

    let mut current_park = String::new();
    for trail in &filtered {
        if trail.park != current_park {
            current_park = trail.park.clone();
            println!("\n{} {}", Icons::TRAIL.green(), current_park.bold());
        }
        print_trail_info(trail, true)?;
    }

    Ok(())
}

fn handle_park(park_name: &str) -> Result<()> {
    let trails = load_trails()?;
    let park_trails: Vec<_> = if park_name.len() <= 4 {
        get_trails_by_park(&trails, park_name)
    } else {
        trails
            .iter()
            .filter(|t| t.park.to_lowercase().contains(&park_name.to_lowercase()))
            .collect()
    };

    if park_trails.is_empty() {
        println!(
            "{}",
            format!("No trails found for park: {}", park_name).yellow()
        );
        return Ok(());
    }

    let mut current_park = String::new();
    for trail in &park_trails {
        if trail.park != current_park {
            current_park = trail.park.clone();
            println!("\n{} {}", Icons::TRAIL.green(), current_park.bold());
        }
        print_trail_info(trail, true)?;
    }

    Ok(())
}

fn handle_trail(trail_name: &str) -> Result<()> {
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

fn print_trail_info(trail: &trails::Trail, show_weather: bool) -> Result<()> {
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

fn handle_card(trail_name: &str) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;
    card::print_card(trail)?;
    Ok(())
}

fn handle_gpx(trail_name: &str, output: Option<&str>) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;

    let default_path = format!("{}.gpx", trail.name.replace(' ', "_").to_lowercase());
    let output_path = output.unwrap_or(&default_path);
    gpx::export_gpx(trail, output_path)?;

    println!("{} Exported: {}", Icons::SUCCESS.green(), output_path);
    println!("   Trail: {} ({:.0}km)", trail.name, trail.length_km);
    println!("   Points: {}", trail.coordinates_wgs84.len());
    println!("   Ready for: Gaia GPS, OsmAnd, AllTrails");

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
