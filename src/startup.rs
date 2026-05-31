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

fn get_arch_display() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("uname").arg("-m").output() {
            if let Ok(arch) = String::from_utf8(output.stdout) {
                let arch = arch.trim();
                if !arch.is_empty() {
                    if arch == "armv7l" || arch.starts_with("armv7") {
                        return "armv7".to_string();
                    } else if arch == "aarch64" {
                        return "arm64".to_string();
                    }
                    return arch.to_string();
                }
            }
        }
    }
    
    let arch = std::env::consts::ARCH;
    if arch == "arm" { "armv7".to_string() } else { arch.to_string() }
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
            let arch_display = get_arch_display();
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
            let arch_display = get_arch_display();
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

/// Run the full bootstrap sequence: initialize the service, collect events,
/// animate the bootstrap screen, and return the ready TaskService.
pub async fn bootstrap<B: Backend>(
    terminal: &mut Terminal<B>,
    db_path: &str,
) -> Result<crate::service::TaskService, Box<dyn std::error::Error>> {
    use std::time::{Duration, Instant};

    let boot_start = Instant::now();

    // Show a minimal bootstrap screen while the service initializes
    draw_bootstrap(terminal, &[
        BootstrapStep { label: "Open SQLite database", completed: false, elapsed_ms: None },
    ], false, None, None)?;

    // Initialize service — real events are fired through ctx.send_event()
    // and captured in the UnifiedLogBuffer.
    let db_open_start = Instant::now();
    let service = crate::service::TaskService::new(db_path).await?;
    let total_elapsed = boot_start.elapsed();

    // Collect real bootstrap events and build the step list dynamically
    let bootstrap_events = service.drain_bootstrap_events();
    let db_open_ms = db_open_start.elapsed().as_secs_f64() * 1000.0;

    // Compute summary counts from event messages
    let mut tables_created = 0usize;
    let mut tables_verified = 0usize;
    let mut fields_added = 0usize;
    let mut seeds = 0usize;
    for (msg, _) in &bootstrap_events {
        if msg.starts_with("Create ") {
            tables_created += 1;
        } else if msg.starts_with("Verified ") {
            tables_verified += 1;
        } else if msg.starts_with("  + field ") {
            fields_added += 1;
        } else if msg.starts_with("Seed ") {
            seeds += 1;
        }
    }
    let entity_count = tables_created + tables_verified;
    let mut summary_parts = Vec::new();
    summary_parts.push(format!("{} entities", entity_count));
    if tables_created > 0 {
        summary_parts.push(format!("{} tables created", tables_created));
    }
    if tables_verified > 0 {
        summary_parts.push(format!("{} tables verified", tables_verified));
    }
    if fields_added > 0 {
        summary_parts.push(format!("{} fields added", fields_added));
    }
    if seeds > 0 {
        summary_parts.push(format!("{} seeds", seeds));
    }
    let summary = summary_parts.join(", ");

    // Build steps: "Open SQLite database" + "N entities discovered" + events + "Startup complete"
    let mut final_steps: Vec<BootstrapStep> = Vec::new();
    final_steps.push(BootstrapStep {
        label: "Open SQLite database",
        completed: true,
        elapsed_ms: Some(db_open_ms),
    });

    // Insert "N entities discovered" step
    let discovered_label: &'static str = Box::leak(
        format!("{} entities discovered", entity_count).into_boxed_str()
    );
    final_steps.push(BootstrapStep {
        label: discovered_label,
        completed: true,
        elapsed_ms: None,
    });

    // Leak the strings so we can use &'static str in BootstrapStep
    for (event_msg, elapsed_ms) in &bootstrap_events {
        let label: &'static str = Box::leak(event_msg.clone().into_boxed_str());
        final_steps.push(BootstrapStep {
            label,
            completed: true,
            elapsed_ms: Some(*elapsed_ms),
        });
    }
    final_steps.push(BootstrapStep {
        label: "TeaQL Runtime ready",
        completed: true,
        elapsed_ms: None,
    });

    // Animate the bootstrap steps with checkmarks one by one
    for i in 0..final_steps.len() {
        let display_steps: Vec<BootstrapStep> = final_steps
            .iter()
            .enumerate()
            .map(|(j, s)| BootstrapStep {
                label: s.label,
                completed: j <= i,
                elapsed_ms: if j <= i { s.elapsed_ms } else { None },
            })
            .collect();
        let all_done = i == final_steps.len() - 1;
        draw_bootstrap(
            terminal,
            &display_steps,
            all_done,
            if all_done { Some(total_elapsed) } else { None },
            if all_done { Some(&summary) } else { None },
        )?;
        if !all_done {
            std::thread::sleep(Duration::from_millis(80));
        }
    }
    wait_for_key()?;

    Ok(service)
}
