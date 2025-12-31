use anyhow::{Context, Result};

pub const MONTREAL_LAT: f64 = 45.5017;
pub const MONTREAL_LNG: f64 = -73.5673;

pub fn lambert_to_wgs84(x: f64, y: f64) -> (f64, f64) {
    lambert_to_wgs84_proj(x, y).unwrap_or_else(|_| lambert_to_wgs84_approx(x, y))
}

fn lambert_to_wgs84_proj(x: f64, y: f64) -> Result<(f64, f64)> {
    let transformer = proj::Proj::new_known_crs("EPSG:32198", "EPSG:4326", None)
        .context("Failed to create proj transformer")?;

    let (lng, lat) = transformer
        .convert((x, y))
        .context("Failed to convert coordinates")?;

    Ok((lat, lng))
}

fn lambert_to_wgs84_approx(x: f64, y: f64) -> (f64, f64) {
    const LAT_OFFSET: f64 = 46.0;
    const LNG_OFFSET: f64 = -71.0;
    const METERS_PER_DEGREE: f64 = 111000.0;

    let lat = LAT_OFFSET + y / METERS_PER_DEGREE;
    let lng = LNG_OFFSET + (x - 1700000.0) / (METERS_PER_DEGREE * LAT_OFFSET.to_radians().cos());

    (lat, lng)
}

pub fn distance_km(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;

    let dlat = (lat2 - lat1).to_radians();
    let dlng = (lng2 - lng1).to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlng / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS_KM * c
}

pub fn extract_coordinates(geometry: &serde_json::Value) -> Option<(f64, f64)> {
    let geom_type = geometry.get("type")?.as_str()?;
    let coords = geometry.get("coordinates")?;

    match geom_type {
        "Point" => coords
            .as_array()?
            .get(0..2)
            .and_then(|arr| Some((arr[0].as_f64()?, arr[1].as_f64()?))),
        "LineString" => coords.as_array()?.first()?.as_array().and_then(|point| {
            point
                .get(0..2)
                .and_then(|arr| Some((arr[0].as_f64()?, arr[1].as_f64()?)))
        }),
        _ => coords
            .as_array()?
            .get(0..2)
            .and_then(|arr| Some((arr[0].as_f64()?, arr[1].as_f64()?))),
    }
}
