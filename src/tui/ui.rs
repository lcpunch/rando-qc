use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
};

use super::app::CompareApp;

pub fn draw_compare(frame: &mut Frame, app: &CompareApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(15),
            Constraint::Length(3),
        ])
        .split(frame.size());

    draw_stats_comparison(frame, app, chunks[0]);
    draw_elevation_chart(frame, app, chunks[1]);
    draw_help_bar(frame, chunks[2]);
}

fn draw_stats_comparison(frame: &mut Frame, app: &CompareApp, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let trail1_block = Block::default()
        .title(Span::styled(
            format!(" {} ", app.trail1_name),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let trail1_text = vec![
        Line::from(format!("Park: {}", app.trail1_park)),
        Line::from(format!("Length: {:.1} km", app.trail1_length)),
        Line::from(format!("Difficulty: {}", app.trail1_difficulty)),
        Line::from(""),
        Line::from(format!("Elevation gain: {:.0} m", app.trail1_gain)),
        Line::from(format!("Max elevation: {:.0} m", app.trail1_max)),
        Line::from(format!("Min elevation: {:.0} m", app.trail1_min)),
    ];

    let trail1_para = Paragraph::new(trail1_text)
        .block(trail1_block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(trail1_para, columns[0]);

    let trail2_block = Block::default()
        .title(Span::styled(
            format!(" {} ", app.trail2_name),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let trail2_text = vec![
        Line::from(format!("Park: {}", app.trail2_park)),
        Line::from(format!("Length: {:.1} km", app.trail2_length)),
        Line::from(format!("Difficulty: {}", app.trail2_difficulty)),
        Line::from(""),
        Line::from(format!("Elevation gain: {:.0} m", app.trail2_gain)),
        Line::from(format!("Max elevation: {:.0} m", app.trail2_max)),
        Line::from(format!("Min elevation: {:.0} m", app.trail2_min)),
    ];

    let trail2_para = Paragraph::new(trail2_text)
        .block(trail2_block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(trail2_para, columns[1]);
}

fn draw_elevation_chart(frame: &mut Frame, app: &CompareApp, area: Rect) {
    let mut datasets = Vec::new();

    if !app.trail1_elevation.is_empty() {
        datasets.push(
            Dataset::default()
                .name(Line::from(app.trail1_name.as_str()))
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Cyan))
                .data(&app.trail1_elevation),
        );
    }

    if !app.trail2_elevation.is_empty() {
        datasets.push(
            Dataset::default()
                .name(Line::from(app.trail2_name.as_str()))
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Magenta))
                .data(&app.trail2_elevation),
        );
    }

    if datasets.is_empty() {
        let block = Block::default()
            .title(" Elevation Profile Comparison ")
            .borders(Borders::ALL);
        let text = Paragraph::new("Elevation data unavailable for one or both trails")
            .block(block)
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(text, area);
        return;
    }

    let max_distance = app.trail1_length.max(app.trail2_length).max(1.0);
    let min_elev = app.trail1_min.min(app.trail2_min) - 20.0;
    let max_elev = app.trail1_max.max(app.trail2_max) + 20.0;
    let elev_range = (max_elev - min_elev).max(1.0);
    let min_el = min_elev;
    let max_el = min_elev + elev_range;

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(" Elevation Profile Comparison ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Distance (km)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_distance])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw(format!("{:.0}", max_distance / 2.0)),
                    Span::raw(format!("{:.0}", max_distance)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Elevation (m)")
                .style(Style::default().fg(Color::Gray))
                .bounds([min_el, max_el])
                .labels(vec![
                    Span::raw(format!("{:.0}", min_el)),
                    Span::raw(format!("{:.0}", (min_el + max_el) / 2.0)),
                    Span::raw(format!("{:.0}", max_el)),
                ]),
        );

    frame.render_widget(chart, area);
}

fn draw_help_bar(frame: &mut Frame, area: Rect) {
    let help_text = Line::from(vec![
        Span::styled(" q ", Style::default().bg(Color::DarkGray).fg(Color::White)),
        Span::raw(" Quit  "),
        Span::styled(" ━━ ", Style::default().fg(Color::Cyan)),
        Span::raw(" Trail 1  "),
        Span::styled(" ━━ ", Style::default().fg(Color::Magenta)),
        Span::raw(" Trail 2 "),
    ]);

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());

    frame.render_widget(help, area);
}
