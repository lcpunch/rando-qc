use crate::trails::Difficulty;
use clap::{Parser, Subcommand};
use std::str::FromStr;

fn parse_f64(s: &str) -> Result<f64, String> {
    f64::from_str(s).map_err(|e| format!("Invalid number: {}", e))
}

#[derive(Parser)]
#[command(name = "rando")]
#[command(about = "Quebec Hiking Trail CLI - Find trails in Sépaq parks")]
#[command(allow_negative_numbers = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List trails with optional filters
    List {
        /// Filter by difficulty (facile, intermediaire, difficile)
        #[arg(short, long)]
        difficulty: Option<String>,

        /// Maximum distance from Montreal (km)
        #[arg(long, default_value = None)]
        max_distance: Option<f64>,

        /// Minimum trail length (km)
        #[arg(long, default_value = None)]
        min_length: Option<f64>,

        /// Maximum trail length (km)
        #[arg(long, default_value = None)]
        max_length: Option<f64>,

        /// Filter by park name (partial match)
        #[arg(short, long)]
        park: Option<String>,
    },

    /// Show trails in a specific park
    Park {
        /// Park name or code (e.g., jacques-cartier, jac, mot)
        name: String,
    },

    /// Show details for a specific trail
    Trail {
        /// Trail name (partial match)
        name: String,
    },

    /// Update cached trail data
    Update,

    /// Display trail info card
    Card {
        /// Trail name (partial match)
        name: String,
    },

    /// Export trail to GPX file
    Gpx {
        /// Trail name (partial match)
        name: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Show 7-day weather forecast for a trail
    Weather {
        /// Trail name (partial match)
        trail: String,
        /// Show 7-day forecast with best day recommendation
        #[arg(long)]
        week: bool,
    },

    /// Find trails near a location
    Nearby {
        /// Latitude
        #[arg(long, value_parser = parse_f64, allow_hyphen_values = true)]
        lat: Option<f64>,
        /// Longitude
        #[arg(long, value_parser = parse_f64, allow_hyphen_values = true)]
        lng: Option<f64>,
        /// Park name (alternative to coordinates)
        #[arg(long)]
        park: Option<String>,
        /// Search radius in km
        #[arg(long, default_value = "50", value_parser = parse_f64)]
        radius: f64,
    },

    /// Compare two trails side by side
    Compare {
        /// First trail name
        trail1: String,
        /// Second trail name
        trail2: String,
    },

    /// Pick a random trail
    Random {
        /// Filter by difficulty
        #[arg(long)]
        difficulty: Option<String>,
        /// Maximum distance from Montreal (km)
        #[arg(long)]
        max_distance: Option<f64>,
    },

    /// Log a completed hike
    Log {
        /// Trail name (partial match)
        trail: String,
        /// Duration (e.g., "4h30m", "2h15m")
        #[arg(long)]
        time: Option<String>,
        /// Date (YYYY-MM-DD format)
        #[arg(long)]
        date: Option<String>,
        /// Notes about the hike
        #[arg(long)]
        notes: Option<String>,
    },

    /// Show personal hiking statistics
    Stats,

    /// Show current hiking streak
    Streak,

    /// Check if you can finish trail before dark
    Daylight {
        /// Trail name (partial match)
        trail: String,
    },

    /// Generate gear checklist for a trail
    Checklist {
        /// Trail name (partial match)
        trail: String,
    },

    /// Show active hunting seasons
    Hunt,

    /// Check for park alerts
    Alerts,

    /// Generate shareable trail info
    Share {
        /// Trail name (partial match)
        trail: String,
    },
}

impl Commands {
    pub fn get_difficulty(&self) -> Option<Difficulty> {
        match self {
            Commands::List { difficulty, .. } => {
                difficulty.as_ref().and_then(|d| Difficulty::from_str(d))
            }
            _ => None,
        }
    }

    pub fn get_max_distance(&self) -> Option<f64> {
        match self {
            Commands::List { max_distance, .. } => *max_distance,
            _ => None,
        }
    }

    pub fn get_min_length(&self) -> Option<f64> {
        match self {
            Commands::List { min_length, .. } => *min_length,
            _ => None,
        }
    }

    pub fn get_max_length(&self) -> Option<f64> {
        match self {
            Commands::List { max_length, .. } => *max_length,
            _ => None,
        }
    }

    pub fn get_park_name(&self) -> Option<&str> {
        match self {
            Commands::List { park, .. } => park.as_deref(),
            _ => None,
        }
    }
}
