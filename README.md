# rando-qc

A CLI tool to find hiking trails in Quebec's Sépaq parks with weather and conditions.

<img width="734" height="627" alt="image" src="https://github.com/user-attachments/assets/32c99aa5-9c43-44fa-9302-de6a0200774e" />

## Installation

```bash
cargo build --release
```

## Commands

- `rando list` - List trails with optional filters
- `rando park <name>` - Show trails in a specific park
- `rando trail <name>` - Show details for a specific trail
- `rando card <name>` - Display trail info card
- `rando gpx <name>` - Export trail to GPX file
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

## Park

Show all trails in a park:

```bash
rando park jac
rando park jacques-cartier
```

## Trail

Show detailed info for a trail:

```bash
rando trail "Le Scotora"
```

## Card

Display a formatted info card for a trail:

```bash
rando card "Le Scotora"
```

## GPX

Export trail to GPX format:

```bash
rando gpx "Le Scotora"
rando gpx "Le Scotora" -o scotora.gpx
```

## Update

Download fresh trail data:

```bash
rando update
```
