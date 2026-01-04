use crate::services::elevation::fetch_elevation;
use crate::trails::Trail;
use anyhow::Result;
use std::fmt::Write;
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

    let difficulty_str = trail
        .difficulty
        .map(|d| d.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let mut gpx = String::new();
    writeln!(gpx, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    writeln!(gpx, "<gpx version=\"1.1\" creator=\"rando-qc\">")?;
    writeln!(gpx, "  <metadata>")?;
    writeln!(gpx, "    <name>{}</name>", escape_xml(&trail.name))?;
    writeln!(
        gpx,
        "    <desc>{} - {} - {:.1}km</desc>",
        escape_xml(&trail.park),
        escape_xml(&difficulty_str),
        trail.length_km
    )?;
    writeln!(gpx, "  </metadata>")?;
    writeln!(gpx, "  <trk>")?;
    writeln!(gpx, "    <name>{}</name>", escape_xml(&trail.name))?;
    writeln!(gpx, "    <trkseg>")?;

    for (i, (lat, lng)) in trail.coordinates_wgs84.iter().enumerate() {
        let ele = elevations.get(i).unwrap_or(&0.0);
        writeln!(
            gpx,
            "      <trkpt lat=\"{:.6}\" lon=\"{:.6}\"><ele>{:.1}</ele></trkpt>",
            lat, lng, ele
        )?;
    }

    writeln!(gpx, "    </trkseg>")?;
    writeln!(gpx, "  </trk>")?;
    writeln!(gpx, "</gpx>")?;

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
