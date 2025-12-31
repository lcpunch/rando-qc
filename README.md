# rando-qc

A CLI tool to find hiking trails in Quebec's Sépaq parks with weather and conditions.

<img width="734" height="627" alt="image" src="https://github.com/user-attachments/assets/32c99aa5-9c43-44fa-9302-de6a0200774e" />

## Installation

```bash
cargo build --release
```

## Commands

### Discovery & Planning
- `rando list` - List trails with optional filters
- `rando park <name>` - Show trails in a specific park
- `rando trail <name>` - Show details for a specific trail
- `rando card <name>` - Display trail info card
- `rando gpx <name>` - Export trail to GPX file
- `rando weather <trail> [--week]` - Show weather forecast (current or 7-day)
- `rando nearby [--lat <lat>] [--lng <lng>] [--park <name>] [--radius <km>]` - Find trails near a location
- `rando compare <trail1> <trail2>` - Compare two trails side by side
- `rando random [--difficulty <diff>] [--max-distance <km>]` - Pick a random trail

### Personal Tracking
- `rando log <trail> [--time <duration>] [--date <date>] [--notes <text>]` - Log a completed hike
- `rando stats` - Show personal hiking statistics
- `rando streak` - Show current hiking streak

### Safety & Conditions
- `rando daylight <trail>` - Check if you can finish before dark
- `rando checklist <trail>` - Generate gear checklist based on conditions
- `rando hunt` - Show active hunting seasons in Quebec
- `rando alerts` - Check for park alerts

### Sharing
- `rando share <trail>` - Generate shareable info with QR code

### Data Management
- `rando update` - Update cached trail data

## List Trails

Filter trails by difficulty, distance, length, and park:

```bash
# Easy trails within 100km of Montreal
rando list --difficulty facile --max-distance 100

# Trails between 5-15km
rando list --min-length 5 --max-length 15

# Trails in a specific park
rando list --park jacques-cartier
```

Options:
- `--difficulty, -d`: facile, intermediaire, difficile
- `--max-distance`: Maximum distance from Montreal (km)
- `--min-length`: Minimum trail length (km)
- `--max-length`: Maximum trail length (km)
- `--park, -p`: Filter by park name

## Weather

Show current weather or 7-day forecast:

```bash
# Current weather
rando weather "Le Scotora"

# 7-day forecast with best day recommendations
rando weather "Le Scotora" --week
```

## Nearby

Find trails near coordinates or a park:

```bash
# By coordinates (note: use --lng for negative longitudes)
rando nearby --lat 46.8530 --lng -71.2958 --radius 30

# By park name
rando nearby --park "Jacques-Cartier" --radius 20
```

## Compare

Compare two trails side by side:

```bash
rando compare "Le Scotora" "Les Loups"
```

## Random

Pick a random trail with optional filters:

```bash
rando random --difficulty facile --max-distance 100
```

## Log

Log a completed hike:

```bash
# Basic log
rando log "Le Scotora"

# With details
rando log "Le Scotora" --time 4h30m --date 2024-12-28 --notes "Beautiful day, saw a moose"
```

## Stats

View your hiking statistics:

```bash
rando stats
```

Shows total hikes, distance, time, parks visited, and monthly breakdown.

## Streak

Track your hiking streak:

```bash
rando streak
```

Shows current streak, longest streak, and monthly progress.

## Daylight

Check if you can complete a trail before dark:

```bash
rando daylight "Le Scotora"
```

## Checklist

Generate a gear checklist based on trail and conditions:

```bash
rando checklist "Le Scotora"
```

**Gear calculation:**
- Water: `(estimated_hours / 2)` liters, minimum 1L
- Headlamp: Added if daylight hours < estimated hours + 1
- Clothing: Based on season (winter: Nov-Mar, summer: Jun-Aug)
- Rain gear: Added if weather code indicates rain (61-82)

**Conditions rating:**
- **Bad**: Thunderstorm OR (rain > 0.5mm AND wind > 20 km/h)
- **Okay**: Rain > 0.5mm OR wind > 20 km/h
- **Excellent**: Clear/partly cloudy (code ≤ 3) AND wind < 15 km/h
- **Good**: All other conditions

**Weather codes:**
- 0: Clear sky
- 1-3: Partly cloudy
- 45, 48: Foggy
- 51, 53, 55: Drizzle
- 61, 63, 65: Rain
- 71, 73, 75: Snow
- 80-82: Rain showers
- 85, 86: Snow showers
- 95: Thunderstorm
- 96, 99: Thunderstorm with hail

## Hunt

Check active hunting seasons:

```bash
rando hunt
```

## Alerts

View park alert information:

```bash
rando alerts
```

## Share

Generate shareable trail info with QR code:

```bash
rando share "Le Scotora"
```

Creates a text file with trail info and displays a QR code.

## Update

Download fresh trail data:

```bash
rando update
```
