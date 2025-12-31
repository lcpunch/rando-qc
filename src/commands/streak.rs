use crate::data::logs::load_logs;
use crate::icons::Icons;
use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};
use colored::Colorize;

pub fn handle_streak() -> Result<()> {
    let logs = load_logs()?;

    if logs.hikes.is_empty() {
        println!(
            "\n{} No hikes logged yet. Start logging with: rando log <trail>",
            Icons::STREAK.yellow()
        );
        return Ok(());
    }

    // Parse dates and group by week
    let mut week_hikes: std::collections::HashSet<(i32, u32)> = std::collections::HashSet::new();
    let mut all_weeks: Vec<(i32, u32)> = Vec::new();

    for hike in &logs.hikes {
        if let Ok(date) = NaiveDate::parse_from_str(&hike.date, "%Y-%m-%d") {
            let week = date.iso_week();
            let week_key = (week.year(), week.week());
            week_hikes.insert(week_key);
            all_weeks.push(week_key);
        }
    }

    all_weeks.sort();
    all_weeks.dedup();

    // Calculate current streak
    let now = Local::now();
    let current_week = now.iso_week();
    let current_week_key = (current_week.year(), current_week.week());

    let mut current_streak = 0;
    let mut check_week = current_week_key;

    loop {
        if week_hikes.contains(&check_week) {
            current_streak += 1;
            // Go to previous week
            if check_week.1 > 1 {
                check_week.1 -= 1;
            } else {
                check_week.0 -= 1;
                check_week.1 = 52; // Approximate
            }
        } else {
            break;
        }
    }

    // Find longest streak
    let mut longest_streak = 1;
    let mut current_run = 1;
    for i in 1..all_weeks.len() {
        let prev = all_weeks[i - 1];
        let curr = all_weeks[i];

        // Check if consecutive weeks
        if (curr.0 == prev.0 && curr.1 == prev.1 + 1)
            || (curr.0 == prev.0 + 1 && curr.1 == 1 && prev.1 >= 52)
        {
            current_run += 1;
            longest_streak = longest_streak.max(current_run);
        } else {
            current_run = 1;
        }
    }

    // Find last hike
    let last_hike = logs
        .hikes
        .iter()
        .filter_map(|h| {
            NaiveDate::parse_from_str(&h.date, "%Y-%m-%d")
                .ok()
                .map(|d| (d, h))
        })
        .max_by_key(|(d, _)| *d);

    println!("\n{} Hiking Streak\n", Icons::STREAK);
    println!(
        "  Current streak:  {} week{}",
        current_streak,
        if current_streak != 1 { "s" } else { "" }
    );

    if let Some((date, hike)) = last_hike {
        let days_ago = (Local::now().date_naive() - date).num_days();
        println!(
            "  Last hike:       {} day{} ago ({})",
            days_ago,
            if days_ago != 1 { "s" } else { "" },
            hike.trail_name.bold()
        );
    }

    println!(
        "  Longest streak:  {} week{}",
        longest_streak,
        if longest_streak != 1 { "s" } else { "" }
    );

    // This month progress
    let this_year = now.year();
    let month_weeks: Vec<_> = all_weeks.iter().filter(|(y, _)| *y == this_year).collect();

    let weeks_this_month = month_weeks.len();
    println!("\n  This month:      {}/4 weeks", weeks_this_month);

    if current_streak > 0 {
        println!(
            "\n  {} Keep it going! Hike this week to continue your streak.",
            Icons::SUCCESS.green()
        );
    } else {
        println!(
            "\n  {} Start a new streak by logging a hike this week!",
            Icons::STREAK.yellow()
        );
    }

    Ok(())
}
