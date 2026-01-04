use crate::cache;
use crate::geo::{
    MONTREAL_LAT, MONTREAL_LNG, distance_km, extract_all_coordinates, lambert_to_wgs84,
};
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::hash_map::Entry;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Trail {
    pub name: String,
    pub park: String,
    pub park_code: String,
    pub difficulty: Option<Difficulty>,
    pub length_km: f64,
    pub lat: f64,
    pub lng: f64,
    pub distance_from_mtl: f64,
    pub coordinates_wgs84: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Facile,
    Intermediaire,
    Difficile,
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "facile" => Ok(Difficulty::Facile),
            "intermédiaire" | "intermediaire" => Ok(Difficulty::Intermediaire),
            "difficile" => Ok(Difficulty::Difficile),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Facile => write!(f, "Facile"),
            Difficulty::Intermediaire => write!(f, "Intermédiaire"),
            Difficulty::Difficile => write!(f, "Difficile"),
        }
    }
}

pub fn load_trails() -> Result<Vec<Trail>> {
    if !cache::trail_data_exists() {
        cache::download_trail_data()?;
    }

    let data = cache::read_trail_data()?;
    let json: Value = serde_json::from_str(&data).context("Failed to parse trail data JSON")?;

    let features = json
        .get("features")
        .and_then(|v| v.as_array())
        .context("Invalid GeoJSON format: missing features array")?;

    let mut trail_map: std::collections::HashMap<(String, String), Trail> =
        std::collections::HashMap::new();

    for feature in features {
        let props = match feature.get("properties").and_then(|v| v.as_object()) {
            Some(p) => p,
            None => continue,
        };

        let geometry = match feature.get("geometry") {
            Some(g) => g,
            None => continue,
        };

        let name = props
            .get("Toponyme1")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .trim()
            .to_string();

        if name.is_empty() || name == "Unknown" {
            continue;
        }

        let park = props
            .get("Nom_etab")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let park_code = props
            .get("Code_etab")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let difficulty = props
            .get("Niv_diff")
            .and_then(|v| v.as_str())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<Difficulty>().ok());

        let length_m = props
            .get("Shape_Leng")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let length_km = length_m / 1000.0;

        let coordinates_lambert = extract_all_coordinates(geometry);
        if coordinates_lambert.is_empty() {
            continue;
        }

        let coordinates_wgs84: Vec<(f64, f64)> = coordinates_lambert
            .iter()
            .map(|&(x, y)| lambert_to_wgs84(x, y))
            .collect();

        let (lat, lng) = coordinates_wgs84[0];
        let distance_from_mtl = distance_km(lat, lng, MONTREAL_LAT, MONTREAL_LNG);

        let trail = Trail {
            name: name.clone(),
            park: park.clone(),
            park_code,
            difficulty,
            length_km,
            lat,
            lng,
            distance_from_mtl,
            coordinates_wgs84,
        };

        let key = (name, park);
        match trail_map.entry(key) {
            Entry::Occupied(mut e) if trail.length_km > e.get().length_km => {
                e.insert(trail);
            }
            Entry::Vacant(e) => {
                e.insert(trail);
            }
            _ => {}
        }
    }

    let mut trails: Vec<Trail> = trail_map.into_values().collect();
    trails.sort_by(|a, b| a.park.cmp(&b.park).then_with(|| a.name.cmp(&b.name)));

    Ok(trails)
}

pub fn filter_trails(
    trails: &[Trail],
    difficulty: Option<Difficulty>,
    max_distance: Option<f64>,
    min_length: Option<f64>,
    max_length: Option<f64>,
    park_name: Option<&str>,
) -> Vec<Trail> {
    trails
        .iter()
        .filter(|trail| {
            if let Some(diff) = difficulty
                && trail.difficulty != Some(diff)
            {
                return false;
            }

            if let Some(max_dist) = max_distance
                && trail.distance_from_mtl > max_dist
            {
                return false;
            }

            if let Some(min_len) = min_length
                && trail.length_km < min_len
            {
                return false;
            }

            if let Some(max_len) = max_length
                && trail.length_km > max_len
            {
                return false;
            }

            if let Some(park) = park_name
                && !trail.park.to_lowercase().contains(&park.to_lowercase())
            {
                return false;
            }

            true
        })
        .cloned()
        .collect()
}

pub fn find_trail_by_name<'a>(trails: &'a [Trail], name: &str) -> Option<&'a Trail> {
    trails
        .iter()
        .find(|t| t.name.to_lowercase().contains(&name.to_lowercase()))
}

pub fn get_trails_by_park<'a>(trails: &'a [Trail], park_code: &str) -> Vec<&'a Trail> {
    trails
        .iter()
        .filter(|t| t.park_code.eq_ignore_ascii_case(park_code))
        .collect()
}
