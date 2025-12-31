use crate::icons::Icons;
use crate::trails::Trail;
use anyhow::Result;
use chrono::{Datelike, Local};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
};
use std::f64::consts::PI;
use std::io::{self, stdout};

pub fn print_card(trail: &Trail) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_card_ui(&mut terminal, trail);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_card_ui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, trail: &Trail) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, trail))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => break,
                _ => {}
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, trail: &Trail) {
    let size = f.size();
    let vertical = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let horizontal = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(vertical[1]);

    let card_area = horizontal[1];

    let card_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(8),
            Constraint::Length(4),
            Constraint::Length(6),
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(card_area);

    let name_upper = trail.name.to_uppercase();
    let difficulty_display =
        if trail.difficulty.trim().is_empty() || trail.difficulty.eq_ignore_ascii_case("Unknown") {
            "Non spécifié"
        } else {
            &trail.difficulty
        };

    let estimated_hours = (trail.length_km / 3.0).ceil() as u32;
    let estimated_time = if estimated_hours >= 2 {
        format!("{}-{}h", estimated_hours - 1, estimated_hours + 1)
    } else {
        "1-2h".to_string()
    };

    let (sunrise, sunset, daylight) = calculate_sun_times(trail.lat, trail.lng);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(Color::Cyan));

    let title = Paragraph::new(name_upper.as_str())
        .block(title_block.clone().title(" TRAIL INFO "))
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, card_layout[0]);

    let park_text = Line::from(vec![
        Span::styled("Park: ", Style::default().fg(Color::Gray)),
        Span::styled(&trail.park, Style::default().fg(Color::White)),
    ]);
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let info = Paragraph::new(vec![
        park_text,
        Line::from(vec![
            Span::styled("Distance: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}km", trail.length_km),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Difficulty: ", Style::default().fg(Color::Gray)),
            Span::styled(difficulty_display, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("Estimated time: ", Style::default().fg(Color::Gray)),
            Span::styled(&estimated_time, Style::default().fg(Color::White)),
        ]),
    ])
    .block(info_block)
    .alignment(Alignment::Left);
    f.render_widget(info, card_layout[1]);

    let (elevation_data, total_gain, total_loss) = generate_elevation_data(trail);
    let elevation_sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} Elevation Profile ", Icons::ELEVATION))
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .data(&elevation_data)
        .style(Style::default().fg(Color::Green))
        .max(100);
    f.render_widget(elevation_sparkline, card_layout[2]);

    let stats_text = Line::from(vec![
        Span::styled(
            format!("↑ Total gain: {}m  ", total_gain),
            Style::default().fg(Color::Green),
        ),
        Span::styled(
            format!("↓ Total loss: {}m", total_loss),
            Style::default().fg(Color::Red),
        ),
    ]);
    let stats = Paragraph::new(stats_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(stats, card_layout[3]);

    let sun_text = vec![
        Line::from(vec![
            Span::styled(
                format!("{} Today: ", Icons::SUN),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                format!("Sunrise {} • Sunset {}", sunrise, sunset),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("   Daylight: ", Style::default().fg(Color::Gray)),
            Span::styled(daylight, Style::default().fg(Color::White)),
        ]),
    ];
    let sun = Paragraph::new(sun_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);
    f.render_widget(sun, card_layout[4]);

    let emergency_text = vec![
        Line::from(vec![Span::styled(
            format!("{} Emergency: ", Icons::ALERT),
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![
            Span::styled("   Park: ", Style::default().fg(Color::Gray)),
            Span::styled("418-848-3169", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("   SQ: ", Style::default().fg(Color::Gray)),
            Span::styled("*4141 (cell) / 310-4141", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("   Coords: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.4}, {:.4}", trail.lat, trail.lng),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("   Hospital: ", Style::default().fg(Color::Gray)),
            Span::styled("CHU de Québec (47km)", Style::default().fg(Color::White)),
        ]),
    ];
    let emergency = Paragraph::new(emergency_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);
    f.render_widget(emergency, card_layout[5]);

    if !trail.park_code.is_empty() {
        let link_text = Line::from(vec![
            Span::styled(
                format!("{} ", Icons::LINK),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                format!("sepaq.com/pq/{}/", trail.park_code.to_lowercase()),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]);
        let link = Paragraph::new(link_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);
        f.render_widget(link, card_layout[6]);
    }

    let now = Local::now();
    let footer_text = Line::from(vec![
        Span::styled("Generated: ", Style::default().fg(Color::Gray)),
        Span::styled(
            now.format("%Y-%m-%d %H:%M").to_string(),
            Style::default().fg(Color::White),
        ),
    ]);
    let footer = Paragraph::new(footer_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);
    f.render_widget(footer, card_layout[7]);

    let help_text = Line::from(vec![
        Span::styled("Press ", Style::default().fg(Color::Gray)),
        Span::styled(
            "q",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(", ", Style::default().fg(Color::Gray)),
        Span::styled(
            "ESC",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(", or ", Style::default().fg(Color::Gray)),
        Span::styled(
            "Enter",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" to exit", Style::default().fg(Color::Gray)),
    ]);
    let help = Paragraph::new(help_text).alignment(Alignment::Center);
    f.render_widget(help, vertical[2]);
}

fn generate_elevation_data(trail: &Trail) -> (Vec<u64>, u32, u32) {
    let num_points = 30;
    let mut data = Vec::new();
    let mut elevations = Vec::new();

    let seed = simple_hash(&trail.name);
    let difficulty_multiplier = match trail.difficulty.to_lowercase().as_str() {
        "difficile" => 1.5,
        "intermédiaire" | "intermediaire" => 1.2,
        _ => 1.0,
    };

    let base_elevation = 200.0;
    let max_elevation_gain = 400.0 * difficulty_multiplier;
    let length_factor = (trail.length_km / 10.0).clamp(0.5, 2.0);

    for i in 0..num_points {
        let t = i as f64 / (num_points - 1) as f64;
        let progress = t * trail.length_km;

        let noise1 = (seed as f64 * 0.1 + progress * 0.3).sin();
        let noise2 = (seed as f64 * 0.2 + progress * 0.5).cos();
        let noise3 = (seed as f64 * 0.3 + progress * 0.7).sin();

        let elevation_variation = noise1 * 0.4 + noise2 * 0.3 + noise3 * 0.3;

        let peak_position = 0.4 + (seed % 100) as f64 / 200.0;
        let peak_effect = (-((t - peak_position) * 3.0).powi(2)).exp() * 0.6;

        let elevation = base_elevation
            + elevation_variation * max_elevation_gain * length_factor
            + peak_effect * max_elevation_gain * 0.8;

        elevations.push(elevation);

        let normalized = ((elevation / (base_elevation + max_elevation_gain * 2.0)) * 100.0)
            .clamp(5.0, 95.0) as u64;
        data.push(normalized);
    }

    let mut total_gain = 0.0;
    let mut total_loss = 0.0;

    for i in 1..elevations.len() {
        let diff = elevations[i] - elevations[i - 1];
        if diff > 0.0 {
            total_gain += diff;
        } else {
            total_loss += diff.abs();
        }
    }

    (data, total_gain as u32, total_loss as u32)
}

fn simple_hash(s: &str) -> u32 {
    s.bytes()
        .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
}

fn calculate_sun_times(lat: f64, lng: f64) -> (String, String, String) {
    let now = Local::now();
    let day_of_year = now.ordinal() as f64;
    let lat_rad = lat * PI / 180.0;

    let declination = 23.45 * PI / 180.0 * (PI * 2.0 * (284.0 + day_of_year) / 365.0).sin();
    let hour_angle = (-lat_rad.tan() * declination.tan()).acos();
    let solar_noon = 12.0 + (4.0 * (lng - (-75.0)) / 60.0);

    let sunrise_hour = solar_noon - hour_angle * 12.0 / PI;
    let sunset_hour = solar_noon + hour_angle * 12.0 / PI;

    let sunrise = format_time(sunrise_hour);
    let sunset = format_time(sunset_hour);

    let daylight_hours = (sunset_hour - sunrise_hour) as u32;
    let daylight_mins = ((sunset_hour - sunrise_hour) * 60.0) as u32 % 60;
    let daylight = format!("{}h {}min", daylight_hours, daylight_mins);

    (sunrise, sunset, daylight)
}

fn format_time(hour: f64) -> String {
    let h = hour.floor() as u32;
    let m = ((hour - hour.floor()) * 60.0).round() as u32;
    format!("{:02}:{:02}", h, m)
}
