use super::card::calculate_sun_times;
use crate::icons::Icons;
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;
use chrono::{Datelike, Local};
use colored::Colorize;

pub fn handle_daylight(trail_name: &str) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;

    let estimated_hours = (trail.length_km / 3.0).ceil() as u32;
    let max_estimated_hours = if estimated_hours >= 2 {
        estimated_hours + 1
    } else {
        2
    };
    let estimated_time = if estimated_hours >= 2 {
        format!("{}-{} hours", estimated_hours - 1, estimated_hours + 1)
    } else {
        "1-2 hours".to_string()
    };

    let (sunrise, sunset, daylight) = calculate_sun_times(trail.lat, trail.lng);

    // Parse sunset time (HH:MM format)
    let sunset_parts: Vec<&str> = sunset.split(':').collect();
    let sunset_hour = sunset_parts[0].parse::<u32>().unwrap_or(16);
    let sunset_min = sunset_parts
        .get(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let sunset_total_minutes = sunset_hour * 60 + sunset_min;

    // Calculate latest start time: sunset - max estimated time
    let max_estimated_minutes = max_estimated_hours * 60;
    let latest_start_minutes = sunset_total_minutes.saturating_sub(max_estimated_minutes);
    let latest_start_hour = latest_start_minutes / 60;
    let latest_start_min = latest_start_minutes % 60;

    println!(
        "\n{} Daylight Check: {}\n",
        Icons::DAYLIGHT,
        trail.name.bold()
    );
    println!("  Estimated time:   {}", estimated_time);
    println!("  Today ({:02}):", Local::now().day());
    println!("    Sunrise:        {}", sunrise);
    println!("    Sunset:         {}", sunset);
    println!("    Daylight:       {}\n", daylight);

    // Check if there's enough daylight
    let daylight_hours = daylight
        .split('h')
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(8.0);

    if daylight_hours >= max_estimated_hours as f64 + 2.0 {
        println!(
            "  {} Plenty of time. Start anytime before {:02}:{:02} to finish before sunset.",
            Icons::SUCCESS.green(),
            latest_start_hour,
            latest_start_min
        );
    } else if daylight_hours >= max_estimated_hours as f64 {
        println!(
            "  {} Tight! Start by {:02}:{:02} to finish before sunset.",
            Icons::WARNING.yellow(),
            latest_start_hour,
            latest_start_min
        );
    } else {
        println!(
            "  {} Not enough daylight today. Consider starting very early or choosing a shorter trail.",
            Icons::WARNING.red()
        );
    }

    Ok(())
}
