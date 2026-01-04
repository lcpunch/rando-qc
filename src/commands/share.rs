use crate::conditions::get_park_url;
use crate::icons::Icons;
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;
use colored::Colorize;
use qrcode::QrCode;
use std::fs;

pub fn handle_share(trail_name: &str) -> Result<()> {
    let trails = load_trails()?;
    let trail = find_trail_by_name(&trails, trail_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail_name))?;

    let url = if !trail.park_code.is_empty() {
        get_park_url(&trail.park_code)
    } else {
        "https://www.sepaq.com/".to_string()
    };

    let difficulty_display = trail
        .difficulty
        .map(|d| d.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let hours = (trail.length_km / 3.0).ceil() as u32;
    let estimated_time = if hours >= 2 {
        format!("~{}-{}h", hours - 1, hours + 1)
    } else {
        "~1-2h".to_string()
    };

    println!("\n{} Share: {}\n", Icons::SHARE, trail.name.bold());
    println!("Text (copy/paste):");
    println!("──────────────────");
    println!("{} — {}", trail.name.bold(), trail.park);
    println!(
        "{:.1}km • {} • {}",
        trail.length_km, difficulty_display, estimated_time
    );
    println!(
        "{} {:.0}km from Montreal",
        Icons::LOCATION,
        trail.distance_from_mtl
    );
    println!("{} {}", Icons::LINK, url);
    println!("──────────────────\n");

    // Generate QR code
    let code = QrCode::new(url.as_bytes())?;
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    println!("QR Code (scan to open trail info):");
    println!("┌─────────────────┐");
    for line in string.lines() {
        println!("│ {}│", line);
    }
    println!("└─────────────────┘\n");

    // Save to file
    let filename = format!("{}-share.txt", trail.name.replace(' ', "_").to_lowercase());
    let content = format!(
        "{}\n{}\n{:.1}km • {} • {}\n{} {:.0}km from Montreal\n{} {}",
        trail.name,
        trail.park,
        trail.length_km,
        difficulty_display,
        estimated_time,
        Icons::LOCATION,
        trail.distance_from_mtl,
        Icons::LINK,
        url
    );
    fs::write(&filename, content)?;
    println!("Saved to: {}", filename.bold());

    Ok(())
}
