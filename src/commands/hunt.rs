use crate::data::hunting::{format_season_dates, get_active_seasons, get_upcoming_seasons};
use crate::icons::Icons;
use anyhow::Result;
use chrono::{Datelike, Local};
use colored::Colorize;

pub fn handle_hunt() -> Result<()> {
    let today = Local::now().date_naive();
    let current_year = today.year();

    println!("\n{} Quebec Hunting Seasons\n", Icons::HUNT);

    let active = get_active_seasons();

    if !active.is_empty() {
        println!("  {}  ACTIVE NOW:\n", Icons::WARNING.yellow());

        for season in &active {
            let (start_str, end_str) = format_season_dates(season, current_year);
            println!("     {} ({})", season.animal.bold(), season.zones);
            println!("     {} → {}", start_str, end_str);
            println!("     {}", season.description);
            println!();
        }

        println!("  {} What does this mean?", Icons::INFO);
        println!("     Hunting is active in forests and crown land across Quebec.");
        println!("     National parks are always off-limits to hunters.");
        println!();

        println!("  {} Where is it safe?", Icons::CHECK.green());
        println!("     - All Sépaq national parks — no hunting ever");
        println!("     - Stay on marked trails within parks");
        println!();

        println!("  {} If hiking OUTSIDE parks:", Icons::WARNING.yellow());
        println!("     - Wear bright orange (vest, hat)");
        println!("     - Make noise to alert hunters");
        println!("     - Avoid dawn and dusk (peak hunting times)");
        println!("     - Check specific zones: quebec.ca/chasse");
    } else {
        println!(
            "  {} No major hunting seasons active right now.\n",
            Icons::CHECK.green()
        );

        // Show upcoming seasons
        let upcoming = get_upcoming_seasons();
        if !upcoming.is_empty() {
            println!("  {} Upcoming:", Icons::CALENDAR);
            for season in upcoming.iter().take(3) {
                let (start_str, _) = format_season_dates(season, current_year);
                println!("     {} — starts {}", season.animal.bold(), start_str);
            }
            println!();
        }

        println!("  {} Reminder:", Icons::INFO);
        println!("     National parks are always safe — no hunting allowed.");
    }

    Ok(())
}
