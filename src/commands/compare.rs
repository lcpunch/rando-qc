use crate::cache;
use crate::services::elevation::{fetch_elevation, sample_coordinates};
use crate::tui;
use crate::trails::{find_trail_by_name, load_trails};
use anyhow::Result;

pub fn handle_compare(trail1_name: &str, trail2_name: &str) -> Result<()> {
    let trails = load_trails()?;

    let trail1 = find_trail_by_name(&trails, trail1_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail1_name))?;
    let trail2 = find_trail_by_name(&trails, trail2_name)
        .ok_or_else(|| anyhow::anyhow!("Trail not found: {}", trail2_name))?;

    println!("Fetching elevation data for both trails...");

    let elev1 = get_trail_elevation(trail1)?;
    let elev2 = get_trail_elevation(trail2)?;

    tui::run_compare_tui(trail1, trail2, &elev1, &elev2)?;

    Ok(())
}

fn get_trail_elevation(trail: &crate::trails::Trail) -> Result<Vec<f64>> {
    if let Some(cached) = cache::get_cached_elevation(&trail.name, &trail.park) {
        return Ok(cached);
    }

    let sampled = sample_coordinates(&trail.coordinates_wgs84, 100);
    let elevations = fetch_elevation(&sampled)?;

    let _ = cache::cache_elevation(&trail.name, &trail.park, &elevations);

    Ok(elevations)
}
