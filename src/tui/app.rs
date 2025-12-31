use crate::services::elevation::calculate_elevation_stats;
use crate::trails::Trail;

pub struct CompareApp {
    pub trail1_name: String,
    pub trail2_name: String,
    pub trail1_park: String,
    pub trail2_park: String,
    pub trail1_length: f64,
    pub trail2_length: f64,
    pub trail1_difficulty: String,
    pub trail2_difficulty: String,
    pub trail1_elevation: Vec<(f64, f64)>,
    pub trail2_elevation: Vec<(f64, f64)>,
    pub trail1_gain: f64,
    pub trail2_gain: f64,
    pub trail1_max: f64,
    pub trail2_max: f64,
    pub trail1_min: f64,
    pub trail2_min: f64,
}

impl CompareApp {
    pub fn new(trail1: &Trail, trail2: &Trail, elev1: &[f64], elev2: &[f64]) -> Self {
        let len1 = trail1.length_km;
        let len2 = trail2.length_km;

        let trail1_elevation = normalize_elevation(elev1, len1);
        let trail2_elevation = normalize_elevation(elev2, len2);

        let stats1 = calculate_elevation_stats(elev1);
        let stats2 = calculate_elevation_stats(elev2);

        Self {
            trail1_name: trail1.name.clone(),
            trail2_name: trail2.name.clone(),
            trail1_park: trail1.park.clone(),
            trail2_park: trail2.park.clone(),
            trail1_length: len1,
            trail2_length: len2,
            trail1_difficulty: trail1.difficulty.clone(),
            trail2_difficulty: trail2.difficulty.clone(),
            trail1_gain: stats1.total_gain,
            trail2_gain: stats2.total_gain,
            trail1_max: stats1.max,
            trail2_max: stats2.max,
            trail1_min: stats1.min,
            trail2_min: stats2.min,
            trail1_elevation,
            trail2_elevation,
        }
    }
}

fn normalize_elevation(elevations: &[f64], total_distance: f64) -> Vec<(f64, f64)> {
    if elevations.is_empty() || total_distance <= 0.0 {
        return vec![];
    }

    elevations
        .iter()
        .enumerate()
        .map(|(i, &elev)| {
            let distance = if elevations.len() > 1 {
                (i as f64 / (elevations.len() - 1) as f64) * total_distance
            } else {
                0.0
            };
            (distance, elev)
        })
        .collect()
}
