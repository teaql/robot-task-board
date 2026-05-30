use std::io;

use crossterm::event::{self, Event};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Clear};
use ratatui::Terminal;

const TEAQL_ORANGE: Color = Color::Rgb(230, 126, 34);
const TEAQL_GREEN: Color = Color::Rgb(46, 204, 113);
const DIM_GRAY: Color = Color::Rgb(120, 120, 120);
const SOFT_WHITE: Color = Color::Rgb(220, 220, 220);

/// Center a fixed-size area within a parent rect
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

/// Screen 1: Welcome screen
pub fn draw_welcome<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    terminal.draw(|f| {
        let area = f.size();
        f.render_widget(Clear, area);

        let box_width: u16 = 74;
        let box_height: u16 = 20;
        let center = centered_rect(box_width, box_height, area);

        let lines = vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "TeaQL Robot Task Board",
                Style::default().fg(TEAQL_ORANGE).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "A self-bootstrapping business application powered by TeaQL Runtime.",
                Style::default().fg(SOFT_WHITE),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "docker run --rm -it teaql/robot-task-board:latest",
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("~4.7 MB ", Style::default().fg(TEAQL_GREEN).add_modifier(Modifier::BOLD)),
                Span::styled("Docker Image", Style::default().fg(SOFT_WHITE)),
            ]),
            Line::from(Span::styled(
                "No distro layer",
                Style::default().fg(DIM_GRAY),
            )),
            Line::from(Span::styled(
                "Self-bootstrap SQLite database",
                Style::default().fg(DIM_GRAY),
            )),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "Press any key to start TeaQL Runtime...",
                Style::default().fg(DIM_GRAY).add_modifier(Modifier::ITALIC),
            )),
            Line::from(""),
            Line::from(""),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(TEAQL_ORANGE));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, center);

        // Render sysinfo above the box (e.g. root@MainRouter (armv7))
        if center.y >= 2 {
            let arch_display = if std::env::consts::ARCH == "arm" { "armv7" } else { std::env::consts::ARCH };
            let sysinfo_str = format!("{}@{} ({})", 
                whoami::username().unwrap_or_else(|_| "user".to_string()), 
                whoami::hostname().unwrap_or_else(|_| "host".to_string()), 
                arch_display
            );
            let sysinfo_rect = Rect::new(center.x, center.y - 2, center.width, 1);
            let sysinfo_paragraph = Paragraph::new(sysinfo_str)
                .style(Style::default().fg(DIM_GRAY));
            f.render_widget(sysinfo_paragraph, sysinfo_rect);
        }
    })?;
    Ok(())
}

/// Format microseconds into a right-aligned 5-digit string with µs suffix.
/// e.g. "12345µs", " 1234µs", "  567µs"
fn format_us(elapsed_ms: f64) -> String {
    let us = (elapsed_ms * 1000.0).round() as u64;
    format!("{:>5}µs", us)
}

/// Screen 2: Bootstrap trace screen — draws the current state of bootstrap steps.
/// Format: [✓] 12345µs - label
pub fn draw_bootstrap<B: Backend>(
    terminal: &mut Terminal<B>,
    steps: &[BootstrapStep],
    all_done: bool,
    total_elapsed: Option<std::time::Duration>,
    summary_line: Option<&str>,
) -> io::Result<()> {
    terminal.draw(|f| {
        let area = f.size();
        f.render_widget(Clear, area);

        let box_width: u16 = 74;
        let box_height: u16 = (steps.len() as u16) + 12;
        let center = centered_rect(box_width, box_height, area);

        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(""));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "TeaQL Runtime Bootstrap",
            Style::default().fg(TEAQL_ORANGE).add_modifier(Modifier::BOLD),
        )).alignment(Alignment::Center));
        lines.push(Line::from(""));

        for step in steps {
            if step.completed {
                if let Some(ms) = step.elapsed_ms {
                    // [✓] 12345µs - label
                    let time_str = format_us(ms);
                    lines.push(Line::from(vec![
                        Span::styled("  [", Style::default().fg(DIM_GRAY)),
                        Span::styled("✓", Style::default().fg(TEAQL_GREEN).add_modifier(Modifier::BOLD)),
                        Span::styled("] ", Style::default().fg(DIM_GRAY)),
                        Span::styled(time_str, Style::default().fg(TEAQL_GREEN)),
                        Span::styled(" - ", Style::default().fg(DIM_GRAY)),
                        Span::styled(step.label.to_string(), Style::default().fg(SOFT_WHITE)),
                    ]));
                } else {
                    // [✓]          label  (no time, e.g. "Startup complete")
                    lines.push(Line::from(vec![
                        Span::styled("  [", Style::default().fg(DIM_GRAY)),
                        Span::styled("✓", Style::default().fg(TEAQL_GREEN).add_modifier(Modifier::BOLD)),
                        Span::styled("]            ", Style::default().fg(DIM_GRAY)),
                        Span::styled(step.label.to_string(), Style::default().fg(SOFT_WHITE)),
                    ]));
                }
            } else {
                // [ ]          label  (not completed yet)
                lines.push(Line::from(vec![
                    Span::styled("  [ ]            ", Style::default().fg(DIM_GRAY)),
                    Span::styled(step.label.to_string(), Style::default().fg(DIM_GRAY)),
                ]));
            }
        }

        lines.push(Line::from(""));

        if let Some(total) = total_elapsed {
            let total_us = (total.as_secs_f64() * 1_000_000.0).round() as u64;
            let footer = if let Some(summary) = summary_line {
                format!("{}  —  {}µs total", summary, total_us)
            } else {
                format!("Powered by TeaQL (@teaqlio)  —  {}µs total", total_us)
            };
            lines.push(Line::from(Span::styled(
                footer,
                Style::default().fg(DIM_GRAY).add_modifier(Modifier::ITALIC),
            )).alignment(Alignment::Center));
        } else {
            lines.push(Line::from(Span::styled(
                "Powered by TeaQL (@teaqlio)",
                Style::default().fg(DIM_GRAY).add_modifier(Modifier::ITALIC),
            )).alignment(Alignment::Center));
        }

        lines.push(Line::from(""));

        if all_done {
            lines.push(Line::from(Span::styled(
                "Press any key to enter Robot Task Board...",
                Style::default().fg(DIM_GRAY).add_modifier(Modifier::ITALIC),
            )).alignment(Alignment::Center));
        } else {
            lines.push(Line::from(Span::styled(
                "Bootstrapping...",
                Style::default().fg(DIM_GRAY).add_modifier(Modifier::ITALIC),
            )).alignment(Alignment::Center));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(""));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(TEAQL_ORANGE));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(paragraph, center);

        // Render sysinfo above the box
        if center.y >= 2 {
            let arch_display = if std::env::consts::ARCH == "arm" { "armv7" } else { std::env::consts::ARCH };
            let sysinfo_str = format!("{}@{} ({})", 
                whoami::username().unwrap_or_else(|_| "user".to_string()), 
                whoami::hostname().unwrap_or_else(|_| "host".to_string()), 
                arch_display
            );
            let sysinfo_rect = Rect::new(center.x, center.y - 2, center.width, 1);
            let sysinfo_paragraph = Paragraph::new(sysinfo_str)
                .style(Style::default().fg(DIM_GRAY));
            f.render_widget(sysinfo_paragraph, sysinfo_rect);
        }
    })?;
    Ok(())
}

/// Wait for any key press
pub fn wait_for_key() -> io::Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
        }
    }
}

/// A single bootstrap step with label, completion state, and optional elapsed time (in ms)
pub struct BootstrapStep {
    pub label: &'static str,
    pub completed: bool,
    pub elapsed_ms: Option<f64>,
}
