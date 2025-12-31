use crate::data::logs::{add_hike, parse_duration};
use crate::icons::Icons;
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;
use colored::Colorize;

pub fn handle_log(
    trail_name: &str,
    time: Option<String>,
    date: Option<String>,
    notes: Option<String>,
) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;

    let duration_minutes = time.as_ref().map(|t| parse_duration(t)).transpose()?;

    add_hike(trail, date.clone(), duration_minutes, notes.clone())?;

    println!("\n{} Logged: {}", Icons::SUCCESS.green(), trail.name.bold());
    if let Some(d) = date.as_ref() {
        println!("   Date: {}", d);
    }
    if let Some(t) = time.as_ref() {
        println!("   Time: {}", t);
    }
    if let Some(n) = notes.as_ref() {
        println!("   Notes: {}", n);
    }

    Ok(())
}
