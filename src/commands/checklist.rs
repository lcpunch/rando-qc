use super::card::calculate_sun_times;
use crate::icons::Icons;
use crate::services::weather::get_weather;
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;
use chrono::{Datelike, Local};
use colored::Colorize;

pub fn handle_checklist(trail_name: &str) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;

    let weather = get_weather(trail.lat, trail.lng).ok();
    let (_, _, daylight) = calculate_sun_times(trail.lat, trail.lng);
    let estimated_hours = (trail.length_km / 3.0).ceil();

    let month = Local::now().month();
    let is_winter = month >= 11 || month <= 3;
    let is_summer = month >= 6 && month <= 8;

    println!(
        "\n{} Gear Checklist: {}\n",
        Icons::CHECKLIST,
        trail.name.bold()
    );
    println!(
        "Based on: {}, {:.1}km, ~{:.0}h, {}\n",
        trail.difficulty,
        trail.length_km,
        estimated_hours,
        month_name(month)
    );

    println!("Essentials:");
    let water_liters = (estimated_hours / 2.0).ceil().max(1.0) as u32;
    println!("  ☐ Water ({}L for this length)", water_liters);
    println!("  ☐ Snacks/lunch");
    println!("  ☐ Map/GPS (phone + offline map)");
    println!("  ☐ First aid kit");

    // Check if need headlamp
    let daylight_hours: f64 = daylight
        .split('h')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8.0);
    if daylight_hours < estimated_hours + 1.0 {
        println!("  ☐ Headlamp (short daylight!)");
    }

    if let Some(ref w) = weather {
        println!(
            "\nClothing (current: {:.0}°C, {}):",
            w.temperature,
            w.description()
        );
    } else {
        println!("\nClothing:");
    }

    if is_winter {
        println!("  ☐ Base layer (wool/synthetic)");
        println!("  ☐ Insulating layer");
        println!("  ☐ Windproof/waterproof jacket");
        println!("  ☐ Winter boots with grip");
        println!("  ☐ Warm hat + gloves");
        println!("  ☐ Extra socks");
        println!("\nWinter gear:");
        println!("  ☐ Microspikes or crampons");
        println!("  ☐ Trekking poles");
        println!("  ☐ Hand warmers");
    } else if is_summer {
        println!("  ☐ Lightweight, breathable clothing");
        println!("  ☐ Sun hat");
        println!("  ☐ Sunscreen");
        println!("  ☐ Sunglasses");
    } else {
        println!("  ☐ Layered clothing");
        println!("  ☐ Rain jacket");
        println!("  ☐ Hat");
    }

    if let Some(ref w) = weather {
        if w.weather_code >= 61 && w.weather_code <= 82 {
            println!("  ☐ Rain gear (rain expected)");
        }
    }

    println!("\nEmergency:");
    println!("  ☐ Emergency blanket");
    println!("  ☐ Whistle");
    println!("  ☐ Phone (charged) + battery pack");
    println!("\nPark info saved:");
    println!("  ☐ Emergency: 418-848-3169");

    Ok(())
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}
