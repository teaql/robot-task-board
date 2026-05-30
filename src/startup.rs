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

        let box_width: u16 = 62;
        let box_height: u16 = 17;
        let center = centered_rect(box_width, box_height, area);

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "TeaQL Robot Task Board",
                Style::default().fg(TEAQL_ORANGE).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "A showcase application built with TeaQL.",
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
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(TEAQL_ORANGE));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, center);
    })?;
    Ok(())
}

/// Screen 2: Bootstrap trace screen — draws the current state of bootstrap steps
/// Steps are left-aligned with elapsed time shown for completed steps.
pub fn draw_bootstrap<B: Backend>(
    terminal: &mut Terminal<B>,
    steps: &[BootstrapStep],
    all_done: bool,
    total_elapsed: Option<std::time::Duration>,
) -> io::Result<()> {
    terminal.draw(|f| {
        let area = f.size();
        f.render_widget(Clear, area);

        let box_width: u16 = 62;
        let box_height: u16 = (steps.len() as u16) + 10;
        let center = centered_rect(box_width, box_height, area);

        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "TeaQL Runtime Bootstrap",
            Style::default().fg(TEAQL_ORANGE).add_modifier(Modifier::BOLD),
        )).alignment(Alignment::Center));
        lines.push(Line::from(""));

        for step in steps {
            if step.completed {
                let time_str = if let Some(elapsed) = step.elapsed_ms {
                    format!("  ({:.1}ms)", elapsed)
                } else {
                    String::new()
                };
                lines.push(Line::from(vec![
                    Span::styled("  [", Style::default().fg(DIM_GRAY)),
                    Span::styled("✓", Style::default().fg(TEAQL_GREEN).add_modifier(Modifier::BOLD)),
                    Span::styled("] ", Style::default().fg(DIM_GRAY)),
                    Span::styled(step.label.to_string(), Style::default().fg(SOFT_WHITE)),
                    Span::styled(time_str, Style::default().fg(DIM_GRAY)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  [ ] ", Style::default().fg(DIM_GRAY)),
                    Span::styled(step.label.to_string(), Style::default().fg(DIM_GRAY)),
                ]));
            }
        }

        lines.push(Line::from(""));

        if let Some(total) = total_elapsed {
            lines.push(Line::from(Span::styled(
                format!("Powered by TeaQL (@teaqlio)  —  {:.0}ms total", total.as_secs_f64() * 1000.0),
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

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(TEAQL_ORANGE));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left);

        f.render_widget(paragraph, center);
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

/// A single bootstrap step with label, completion state, and optional elapsed time
pub struct BootstrapStep {
    pub label: &'static str,
    pub completed: bool,
    pub elapsed_ms: Option<f64>,
}

/// Bootstrap step labels matching the real operations
pub const BOOTSTRAP_LABELS: &[&str] = &[
    "Open SQLite database",
    "Create platform_data table",
    "Create task_status_data table",
    "Create task_data table",
    "Create task_execution_log_data table",
    "Seed platform_data",
    "Seed task_status_data",
    "Startup complete",
];
