use crate::data::logs::load_logs;
use crate::icons::Icons;
use anyhow::Result;
use chrono::{Datelike, Local};
use colored::Colorize;
use std::collections::HashMap;

pub fn handle_stats() -> Result<()> {
    let logs = load_logs()?;

    if logs.hikes.is_empty() {
        println!(
            "\n{} No hikes logged yet. Start logging with: rando log <trail>",
            Icons::STATS.yellow()
        );
        return Ok(());
    }

    let total_hikes = logs.hikes.len();
    let total_distance: f64 = logs.hikes.iter().map(|h| h.distance_km).sum();
    let total_minutes: u32 = logs.hikes.iter().filter_map(|h| h.duration_minutes).sum();
    let total_hours = total_minutes / 60;
    let total_mins = total_minutes % 60;

    let parks: std::collections::HashSet<_> = logs.hikes.iter().map(|h| h.park.as_str()).collect();
    let trails: std::collections::HashSet<_> =
        logs.hikes.iter().map(|h| h.trail_name.as_str()).collect();

    let longest = logs.hikes.iter().max_by(|a, b| {
        a.distance_km
            .partial_cmp(&b.distance_km)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let trail_counts: HashMap<&str, usize> =
        logs.hikes
            .iter()
            .map(|h| h.trail_name.as_str())
            .fold(HashMap::new(), |mut acc, name| {
                *acc.entry(name).or_insert(0) += 1;
                acc
            });

    let most_visited = trail_counts.iter().max_by_key(|(_, count)| *count);

    println!("\n{} Your Hiking Stats\n", Icons::STATS);
    println!("  Total hikes:      {}", total_hikes);
    println!("  Total distance:   {:.1} km", total_distance);
    println!("  Total time:       {}h {}min\n", total_hours, total_mins);
    println!("  Parks visited:    {}", parks.len());
    println!("  Trails completed: {}\n", trails.len());

    if let Some(hike) = longest {
        let duration_str = hike
            .duration_minutes
            .map(|m| format!("{}h{}m", m / 60, m % 60))
            .unwrap_or_else(|| "unknown".to_string());
        println!(
            "  Longest hike:     {} ({:.1}km, {})",
            hike.trail_name.bold(),
            hike.distance_km,
            duration_str
        );
    }

    if let Some((trail, count)) = most_visited {
        println!("  Most visited:     {} ({} times)\n", trail.bold(), count);
    }

    // By month
    println!("  By month ({}):", Local::now().year());
    let mut month_counts: HashMap<u32, usize> = HashMap::new();
    for hike in &logs.hikes {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(&hike.date, "%Y-%m-%d") {
            *month_counts.entry(date.month()).or_insert(0) += 1;
        }
    }

    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    for (idx, month_name) in month_names.iter().enumerate() {
        let month_num = (idx + 1) as u32;
        let count = month_counts.get(&month_num).copied().unwrap_or(0);
        let bar = "█".repeat(count.min(10));
        println!("    {} {}", month_name, bar);
    }

    Ok(())
}
