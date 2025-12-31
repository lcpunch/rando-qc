use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

const CACHE_DIR: &str = "rando-qc";
const TRAIL_DATA_FILE: &str = "sentieretel.json";

pub fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".cache")))
        .context("Could not find cache directory")?
        .join(CACHE_DIR);

    fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;

    Ok(cache_dir)
}

pub fn get_trail_data_path() -> Result<PathBuf> {
    Ok(get_cache_dir()?.join(TRAIL_DATA_FILE))
}

pub fn trail_data_exists() -> bool {
    get_trail_data_path().map(|p| p.exists()).unwrap_or(false)
}

pub fn download_trail_data() -> Result<()> {
    let url = "https://www.donneesquebec.ca/recherche/dataset/f5c2e540-4416-4e90-9520-7837b8e31346/resource/81d32e61-16b8-4ccd-a0c4-c34e220e4420/download/sentieretel.json";

    println!("Downloading trail data from Quebec open data...");
    let response = reqwest::blocking::get(url).context("Failed to download trail data")?;

    let data = response.text().context("Failed to read response")?;

    let path = get_trail_data_path()?;
    fs::write(&path, data).context("Failed to write cached trail data")?;

    println!("Trail data cached to: {}", path.display());
    Ok(())
}

pub fn read_trail_data() -> Result<String> {
    let path = get_trail_data_path()?;
    fs::read_to_string(&path).context("Failed to read cached trail data")
}
