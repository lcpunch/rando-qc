use crate::trails::Difficulty;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rando")]
#[command(about = "Quebec Hiking Trail CLI - Find trails in Sépaq parks")]
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
