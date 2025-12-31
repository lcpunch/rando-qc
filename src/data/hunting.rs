use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct HuntingSeason {
    pub animal: &'static str,
    pub description: &'static str,
    pub start_month: u32,
    pub start_day: u32,
    pub end_month: u32,
    pub end_day: u32,
    pub zones: &'static str,
}

const HUNTING_SEASONS: &[HuntingSeason] = &[
    HuntingSeason {
        animal: "Small game",
        description: "hare, grouse, ptarmigan, woodcock",
        start_month: 9,
        start_day: 1,
        end_month: 3,
        end_day: 31,
        zones: "province-wide",
    },
    HuntingSeason {
        animal: "White-tailed deer",
        description: "rifle season",
        start_month: 11,
        start_day: 4,
        end_month: 11,
        end_day: 17,
        zones: "zones 4-10",
    },
    HuntingSeason {
        animal: "Moose",
        description: "rifle season",
        start_month: 9,
        start_day: 20,
        end_month: 10,
        end_day: 26,
        zones: "zones 1-26",
    },
    HuntingSeason {
        animal: "Black bear",
        description: "fall season",
        start_month: 8,
        start_day: 15,
        end_month: 11,
        end_day: 15,
        zones: "zones 1-28",
    },
];

pub fn get_active_seasons() -> Vec<&'static HuntingSeason> {
    let today = Local::now().date_naive();
    HUNTING_SEASONS
        .iter()
        .filter(|s| is_season_active(s, today))
        .collect()
}

pub fn get_upcoming_seasons() -> Vec<&'static HuntingSeason> {
    let today = Local::now().date_naive();
    let mut upcoming: Vec<_> = HUNTING_SEASONS
        .iter()
        .filter(|s| !is_season_active(s, today))
        .collect();

    // Sort by start date
    upcoming.sort_by_key(|s| (s.start_month, s.start_day));
    upcoming
}

fn is_season_active(season: &HuntingSeason, today: NaiveDate) -> bool {
    let year = today.year();

    // Handle seasons that span year boundary (e.g., Sep 2025 - Mar 2026)
    let (start_year, end_year) = if season.end_month < season.start_month {
        // Season crosses year boundary
        if today.month() >= season.start_month {
            (year, year + 1)
        } else {
            (year - 1, year)
        }
    } else {
        (year, year)
    };

    let start =
        NaiveDate::from_ymd_opt(start_year, season.start_month, season.start_day).unwrap_or(today);
    let end = NaiveDate::from_ymd_opt(end_year, season.end_month, season.end_day).unwrap_or(today);

    today >= start && today <= end
}

pub fn format_season_dates(season: &HuntingSeason, current_year: i32) -> (String, String) {
    let (start_year, end_year) = if season.end_month < season.start_month {
        (current_year, current_year + 1)
    } else {
        (current_year, current_year)
    };

    let start_str = format!(
        "{} {}, {}",
        month_abbrev(season.start_month),
        season.start_day,
        start_year
    );

    let end_str = format!(
        "{} {}, {}",
        month_abbrev(season.end_month),
        season.end_day,
        end_year
    );

    (start_str, end_str)
}

fn month_abbrev(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}
