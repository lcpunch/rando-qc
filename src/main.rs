mod cache;
mod cli;
mod commands;
mod conditions;
mod geo;
mod icons;
mod services;
mod trails;

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
    }

    Ok(())
}
