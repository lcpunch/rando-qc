use crate::cli::Cli;
use crate::icons::Icons;
use crate::trails::{Difficulty, filter_trails, load_trails};
use anyhow::Result;
use colored::Colorize;

pub fn handle_list(cli: &Cli) -> Result<()> {
    if let crate::cli::Commands::List {
        difficulty: Some(ref diff_str),
        ..
    } = cli.command
        && diff_str.parse::<Difficulty>().is_err()
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
        super::print_trail_info(trail, true)?;
    }

    Ok(())
}
