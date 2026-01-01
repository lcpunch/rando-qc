mod cache;
mod cli;
mod commands;
mod conditions;
mod data;
mod geo;
mod icons;
mod services;
mod trails;
mod tui;

use crate::icons::Icons;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

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
        Commands::List { .. } => commands::handle_list(&cli)?,
        Commands::Park { name } => commands::handle_park(name)?,
        Commands::Trail { name } => commands::handle_trail(name)?,
        Commands::Card { name } => {
            let trails = trails::load_trails()?;
            let trail = trails::find_trail_by_name(&trails, name)
                .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", name))?;
            commands::print_card(trail)?;
        }
        Commands::Gpx { name, output } => {
            let trails = trails::load_trails()?;
            let trail = trails::find_trail_by_name(&trails, name)
                .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", name))?;

            let default_path = format!("{}.gpx", trail.name.replace(' ', "_").to_lowercase());
            let output_path = output.as_deref().unwrap_or(&default_path);
            commands::export_gpx(trail, output_path)?;

            println!("{} Exported: {}", Icons::SUCCESS.green(), output_path);
            println!("   Trail: {} ({:.0}km)", trail.name, trail.length_km);
            println!("   Points: {}", trail.coordinates_wgs84.len());
            println!("   Ready for: Gaia GPS, OsmAnd, AllTrails");
        }
        Commands::Weather { trail, week } => {
            commands::handle_weather(trail, *week)?;
        }
        Commands::Nearby {
            lat,
            lng,
            park,
            radius,
        } => {
            commands::handle_nearby(*lat, *lng, park.clone(), *radius)?;
        }
        Commands::Compare { trail1, trail2 } => {
            commands::handle_compare(trail1, trail2)?;
        }
        Commands::Random {
            difficulty,
            max_distance,
        } => {
            commands::handle_random(difficulty.clone(), *max_distance)?;
        }
        Commands::Log {
            trail,
            time,
            date,
            notes,
        } => {
            commands::handle_log(trail, time.clone(), date.clone(), notes.clone())?;
        }
        Commands::Stats => {
            commands::handle_stats()?;
        }
        Commands::Streak => {
            commands::handle_streak()?;
        }
        Commands::Daylight { trail } => {
            commands::handle_daylight(trail)?;
        }
        Commands::Checklist { trail } => {
            commands::handle_checklist(trail)?;
        }
        Commands::Hunt => {
            commands::handle_hunt()?;
        }
        Commands::Alerts => {
            commands::handle_alerts()?;
        }
        Commands::Share { trail } => {
            commands::handle_share(trail)?;
        }
    }

    Ok(())
}
