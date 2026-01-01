use crate::trails::Trail;
use anyhow::{Context, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HikeLog {
    pub trail_name: String,
    pub park: String,
    pub date: String, // YYYY-MM-DD
    pub duration_minutes: Option<u32>,
    pub distance_km: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LogsData {
    pub hikes: Vec<HikeLog>,
}

fn get_logs_path() -> Result<PathBuf> {
    let data_dir = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/share")))
        .context("Could not find data directory")?
        .join("rando-qc");

    fs::create_dir_all(&data_dir).context("Failed to create data directory")?;
    Ok(data_dir.join("logs.json"))
}

pub fn load_logs() -> Result<LogsData> {
    let path = get_logs_path()?;
    if !path.exists() {
        return Ok(LogsData::default());
    }

    let data = fs::read_to_string(&path).context("Failed to read logs file")?;
    serde_json::from_str(&data).context("Failed to parse logs JSON")
}

pub fn save_logs(logs: &LogsData) -> Result<()> {
    let path = get_logs_path()?;
    let data = serde_json::to_string_pretty(logs)?;
    fs::write(&path, data).context("Failed to write logs file")
}

pub fn add_hike(
    trail: &Trail,
    date: Option<String>,
    duration_minutes: Option<u32>,
    notes: Option<String>,
) -> Result<()> {
    let mut logs = load_logs()?;

    let date_str = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    let hike = HikeLog {
        trail_name: trail.name.clone(),
        park: trail.park.clone(),
        date: date_str,
        duration_minutes,
        distance_km: trail.length_km,
        notes,
    };

    logs.hikes.push(hike);
    save_logs(&logs)
}

pub fn parse_duration(time_str: &str) -> Result<u32> {
    let mut total_minutes = 0u32;
    let mut current_num = String::new();

    for ch in time_str.chars() {
        if ch.is_ascii_digit() {
            current_num.push(ch);
        } else if ch == 'h' || ch == 'H' {
            if !current_num.is_empty() {
                total_minutes += current_num.parse::<u32>()? * 60;
                current_num.clear();
            }
        } else if (ch == 'm' || ch == 'M') && !current_num.is_empty() {
            total_minutes += current_num.parse::<u32>()?;
            current_num.clear();
        }
    }

    if !current_num.is_empty() {
        total_minutes += current_num.parse::<u32>()?;
    }

    Ok(total_minutes)
}
