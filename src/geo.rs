use proj::Proj;
use std::cell::RefCell;

pub const MONTREAL_LAT: f64 = 45.5017;
pub const MONTREAL_LNG: f64 = -73.5673;

thread_local! {
    static TRANSFORMER: RefCell<Option<Proj>> = RefCell::new(
        Proj::new_known_crs("EPSG:32198", "EPSG:4326", None).ok()
    );
}

pub fn lambert_to_wgs84(x: f64, y: f64) -> (f64, f64) {
    TRANSFORMER.with(|transformer_cell| {
        let transformer_opt = transformer_cell.borrow();
        if let Some(ref transformer) = *transformer_opt
            && let Ok((lng, lat)) = transformer.convert((x, y))
        {
            return (lat, lng);
        }
        drop(transformer_opt);
        lambert_to_wgs84_approx(x, y)
    })
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

pub fn extract_all_coordinates(geometry: &serde_json::Value) -> Vec<(f64, f64)> {
    let geom_type = match geometry.get("type").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let coords = match geometry.get("coordinates") {
        Some(c) => c,
        None => return Vec::new(),
    };

    match geom_type {
        "Point" => {
            if let Some(arr) = coords.as_array()
                && arr.len() >= 2
                && let (Some(x), Some(y)) = (arr[0].as_f64(), arr[1].as_f64())
            {
                return [(x, y)].to_vec();
            }
            Vec::new()
        }
        "LineString" => {
            let mut points = Vec::new();
            if let Some(arr) = coords.as_array() {
                for point in arr {
                    if let Some(point_arr) = point.as_array()
                        && point_arr.len() >= 2
                        && let (Some(x), Some(y)) = (point_arr[0].as_f64(), point_arr[1].as_f64())
                    {
                        points.push((x, y));
                    }
                }
            }
            points
        }
        _ => {
            if let Some(arr) = coords.as_array()
                && arr.len() >= 2
                && let (Some(x), Some(y)) = (arr[0].as_f64(), arr[1].as_f64())
            {
                return vec![(x, y)];
            }
            Vec::new()
        }
    }
}
