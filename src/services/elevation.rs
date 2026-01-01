use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct ElevationResponse {
    elevation: Vec<f64>,
}

/// Fetch elevation for a list of coordinates from Open-Meteo API
/// Coordinates are (lat, lng) pairs
/// Returns elevation in meters for each point
pub fn fetch_elevation(coordinates: &[(f64, f64)]) -> Result<Vec<f64>> {
    if coordinates.is_empty() {
        return Ok(Vec::new());
    }

    // Open-Meteo allows max 100 points per request
    const BATCH_SIZE: usize = 100;
    let mut all_elevations = Vec::with_capacity(coordinates.len());

    for chunk in coordinates.chunks(BATCH_SIZE) {
        let lats: Vec<String> = chunk.iter().map(|(lat, _)| format!("{:.6}", lat)).collect();
        let lngs: Vec<String> = chunk.iter().map(|(_, lng)| format!("{:.6}", lng)).collect();

        let url = format!(
            "https://api.open-meteo.com/v1/elevation?latitude={}&longitude={}",
            lats.join(","),
            lngs.join(",")
        );

        let response: ElevationResponse = reqwest::blocking::get(&url)
            .context("Failed to fetch elevation data")?
            .json()
            .context("Failed to parse elevation response")?;

        all_elevations.extend(response.elevation);
    }

    Ok(all_elevations)
}

/// Sample coordinates to reduce API calls
/// If trail has 500 points, sample ~50 evenly spaced points
pub fn sample_coordinates(coordinates: &[(f64, f64)], max_points: usize) -> Vec<(f64, f64)> {
    if coordinates.len() <= max_points {
        return coordinates.to_vec();
    }

    let step = coordinates.len() as f64 / max_points as f64;
    let mut sampled: Vec<_> = (0..max_points)
        .filter_map(|i| {
            let idx = (i as f64 * step) as usize;
            coordinates.get(idx).copied()
        })
        .collect();

    // Always include last point if not already included
    if let Some(&last) = coordinates.last()
        && sampled.last() != Some(&last)
    {
        sampled.push(last);
    }

    sampled
}

/// Calculate elevation statistics
pub struct ElevationStats {
    pub min: f64,
    pub max: f64,
    pub total_gain: f64,
    pub total_loss: f64,
    pub elevations: Vec<f64>,
}

pub fn calculate_elevation_stats(elevations: &[f64]) -> ElevationStats {
    if elevations.is_empty() {
        return ElevationStats {
            min: 0.0,
            max: 0.0,
            total_gain: 0.0,
            total_loss: 0.0,
            elevations: Vec::new(),
        };
    }

    let min = elevations.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = elevations.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let mut total_gain = 0.0;
    let mut total_loss = 0.0;

    for i in 1..elevations.len() {
        let diff = elevations[i] - elevations[i - 1];
        if diff > 0.0 {
            total_gain += diff;
        } else {
            total_loss += diff.abs();
        }
    }

    ElevationStats {
        min,
        max,
        total_gain,
        total_loss,
        elevations: elevations.to_vec(),
    }
}
