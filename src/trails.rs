use crate::cache;
use crate::geo::{distance_km, extract_coordinates, lambert_to_wgs84, MONTREAL_LAT, MONTREAL_LNG};
use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Trail {
    pub name: String,
    pub park: String,
    pub park_code: String,
    pub difficulty: String,
    pub length_km: f64,
    pub lat: f64,
    pub lng: f64,
    pub distance_from_mtl: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Facile,
    Intermediaire,
    Difficile,
}

impl Difficulty {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "facile" => Some(Difficulty::Facile),
            "intermédiaire" | "intermediaire" => Some(Difficulty::Intermediaire),
            "difficile" => Some(Difficulty::Difficile),
            _ => None,
        }
    }

    pub fn matches(&self, trail_difficulty: &str) -> bool {
        match self {
            Difficulty::Facile => trail_difficulty.eq_ignore_ascii_case("Facile"),
            Difficulty::Intermediaire => {
                trail_difficulty.eq_ignore_ascii_case("Intermédiaire")
                    || trail_difficulty.eq_ignore_ascii_case("Intermediaire")
            }
            Difficulty::Difficile => trail_difficulty.eq_ignore_ascii_case("Difficile"),
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
            .unwrap_or("Unknown")
            .to_string();

        let length_m = props
            .get("Shape_Leng")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let length_km = length_m / 1000.0;

        if let Some((x, y)) = extract_coordinates(geometry) {
            let (lat, lng) = lambert_to_wgs84(x, y);
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
            };

            let key = (name, park);
            match trail_map.get_mut(&key) {
                Some(existing) if trail.length_km > existing.length_km => {
                    *existing = trail;
                }
                None => {
                    trail_map.insert(key, trail);
                }
                _ => {}
            }
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
            if let Some(diff) = difficulty {
                if !diff.matches(&trail.difficulty) {
                    return false;
                }
            }

            if let Some(max_dist) = max_distance {
                if trail.distance_from_mtl > max_dist {
                    return false;
                }
            }

            if let Some(min_len) = min_length {
                if trail.length_km < min_len {
                    return false;
                }
            }

            if let Some(max_len) = max_length {
                if trail.length_km > max_len {
                    return false;
                }
            }

            if let Some(park) = park_name {
                if !trail.park.to_lowercase().contains(&park.to_lowercase()) {
                    return false;
                }
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
