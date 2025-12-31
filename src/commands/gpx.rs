use crate::services::elevation::fetch_elevation;
use crate::trails::Trail;
use anyhow::Result;
use std::fs;

pub fn export_gpx(trail: &Trail, output_path: &str) -> Result<()> {
    println!(
        "Fetching elevation data for {} points...",
        trail.coordinates_wgs84.len()
    );

    let elevations = fetch_elevation(&trail.coordinates_wgs84).unwrap_or_else(|e| {
        eprintln!("Warning: Could not fetch elevation data: {}", e);
        vec![0.0; trail.coordinates_wgs84.len()]
    });

    let mut gpx = String::new();

    gpx.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    gpx.push_str("<gpx version=\"1.1\" creator=\"rando-qc\">\n");
    gpx.push_str("  <metadata>\n");
    gpx.push_str(&format!("    <name>{}</name>\n", escape_xml(&trail.name)));
    gpx.push_str(&format!(
        "    <desc>{} - {} - {:.1}km</desc>\n",
        escape_xml(&trail.park),
        escape_xml(&trail.difficulty),
        trail.length_km
    ));
    gpx.push_str("  </metadata>\n");
    gpx.push_str("  <trk>\n");
    gpx.push_str(&format!("    <name>{}</name>\n", escape_xml(&trail.name)));
    gpx.push_str("    <trkseg>\n");

    for (i, (lat, lng)) in trail.coordinates_wgs84.iter().enumerate() {
        let ele = elevations.get(i).unwrap_or(&0.0);
        gpx.push_str(&format!(
            "      <trkpt lat=\"{:.6}\" lon=\"{:.6}\"><ele>{:.1}</ele></trkpt>\n",
            lat, lng, ele
        ));
    }

    gpx.push_str("    </trkseg>\n");
    gpx.push_str("  </trk>\n");
    gpx.push_str("</gpx>\n");

    fs::write(output_path, gpx)?;
    Ok(())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
