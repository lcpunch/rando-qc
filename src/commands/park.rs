use crate::icons::Icons;
use crate::trails::{get_trails_by_park, load_trails};
use anyhow::Result;
use colored::Colorize;

pub fn handle_park(park_name: &str) -> Result<()> {
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
        super::print_trail_info(trail, true)?;
    }

    Ok(())
}
