use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::App;

pub fn parse_log_line(line: &str) -> Line<'_> {
    if line.len() < 23 || !line.starts_with("202") {
        return Line::from(line.cyan());
    }

    let mut spans = Vec::new();
    
    // 1. Timestamp (Slate gray)
    let timestamp = &line[..23];
    spans.push(Span::styled(timestamp, Style::default().fg(Color::Indexed(244))));

    let mut rest = &line[23..];

    // 2. User ID bracket e.g. -[philip@pid-xxx.tid-x] (Neon Violet)
    if rest.starts_with("-[") {
        if let Some(end) = rest.find(']') {
            let user_part = &rest[..end+1];
            spans.push(Span::styled(user_part, Style::default().fg(Color::Rgb(155, 89, 182)).add_modifier(Modifier::BOLD)));
            rest = &rest[end+1..];
        }
    }

    // 3. Severity e.g. --DEBUG - SqlLogEntry
    if rest.starts_with("--DEBUG - SqlLogEntry") {
        spans.push(Span::styled("--DEBUG - SqlLogEntry", Style::default().fg(Color::Indexed(242))));
        rest = &rest[21..];
    }

    // 4. Comment part and Result summary part
    // Dynamically distinguish between log entries that have comments vs. those that do not.
    if rest.starts_with(" - [") {
        if let Some(end) = rest[4..].find(']') {
            let first_segment = &rest[..end+5];
            let after_first = &rest[end+5..];
            
            if after_first.starts_with(" - [") {
                // If there is another " - [" immediately following, then the first one is the comment!
                spans.push(Span::styled(first_segment, Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD)));
                rest = after_first;
                
                // Now parse the second one as the result summary
                if let Some(end2) = rest[4..].find(']') {
                    let result_part = &rest[..end2+5];
                    spans.push(Span::styled(result_part, Style::default().fg(Color::Rgb(52, 152, 219))));
                    rest = &rest[end2+5..];
                }
            } else {
                // If there is no " - [" following, then this first segment is the result summary (no comment exists)!
                spans.push(Span::styled(first_segment, Style::default().fg(Color::Rgb(52, 152, 219))));
                rest = after_first;
            }
        }
    }

    // 6. SQL statement and elapsed time
    if let Some(took_idx) = rest.rfind(" (took ") {
        let sql = &rest[..took_idx];
        let took = &rest[took_idx..];
        spans.push(Span::styled(sql, Style::default().fg(Color::White)));
        spans.push(Span::styled(took, Style::default().fg(Color::Rgb(231, 76, 60))));
    } else {
        spans.push(Span::styled(rest, Style::default().fg(Color::White)));
    }

    Line::from(spans)
}

pub fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // 1. Log area
            Constraint::Length(3),      // 2. Status statistics
            Constraint::Min(5),         // 3. Columns (Planned, Process, Done)
            Constraint::Length(3),      // 4. Command Line Area
            Constraint::Length(9),      // 5. Command Help Area
        ])
        .split(f.size());

    // 1. Render Log Area (Keeps beautiful syntax-highlighted logs)
    let log_height = chunks[0].height as usize - 2; // Subtract borders
    let skip_count = app.logs.len().saturating_sub(log_height);
    let log_lines: Vec<Line> = app.logs.iter()
        .skip(skip_count)
        .map(|l| parse_log_line(l))
        .collect();

    let search_str = if let Some(ref term) = app.search_term {
        term.as_str()
    } else {
        "None"
    };
    let title_str = format!(
        " Action Logs & Executed SQL | CPU: {} | MEM: {} | Search: {} ",
        app.cpu_model, app.mem_size, search_str
    );

    let log_paragraph = Paragraph::new(log_lines)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title_str)
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::Indexed(240))),
    );
    f.render_widget(log_paragraph, chunks[0]);

    // 2. Render Status Statistics Area (3 equal columns - plain white borders)
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[1]);

    let planned_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Planned tasks count: "),
        Span::styled(
            format!("{}", app.planned_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(planned_stat, stats_chunks[0]);

    let process_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Tasks in Process count: "),
        Span::styled(
            format!("{}", app.process_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(process_stat, stats_chunks[1]);

    let done_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Completed Tasks count: "),
        Span::styled(
            format!("{}", app.done_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(done_stat, stats_chunks[2]);

    // 3. Render task list columns (Planned, Process, Done - plain white borders)
    let col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[2]);

    // Planned Tasks column
    let planned_lines = app
        .planned_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::Indexed(243))),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let planned_list = Paragraph::new(planned_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" PLANNED ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(planned_list, col_chunks[0]);

    // Process Tasks column
    let process_lines = app
        .process_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::Indexed(243))),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let process_list = Paragraph::new(process_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" PROCESS ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(process_list, col_chunks[1]);

    // Done Tasks column
    let done_lines = app
        .done_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::Indexed(243))),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let done_list = Paragraph::new(done_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" DONE ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(done_list, col_chunks[2]);

    // 4. Render Command Line Area (plain white borders)
    let prompt_line = Line::from(vec![
        Span::styled("  >  ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(app.input.clone(), Style::default().fg(Color::White)),
    ]);
    let cmd_input = Paragraph::new(prompt_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Command Line Area ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(cmd_input, chunks[3]);

    // Position and show the cursor inside the input area (adjusted +6: 1 for left border + 5 for prompt arrow)
    f.set_cursor(
        chunks[3].x + 6 + app.input.chars().count() as u16,
        chunks[3].y + 1,
    );

    let help_text = vec![
        Line::from(vec![
            Span::raw("  "),
            Span::styled("add <name>           ", Style::default().fg(Color::White)),
            Span::raw("- Create a new task in Planned status"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("move <id> [s]        ", Style::default().fg(Color::White)),
            Span::raw("- Change status to planned/process/done (default next)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("delete <id>          ", Style::default().fg(Color::White)),
            Span::raw("- Permanently delete task from database"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("search <kw>          ", Style::default().fg(Color::White)),
            Span::raw("- Search tasks by keyword using JSON dynamic EXPR"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("s <kw>               ", Style::default().fg(Color::White)),
            Span::raw("- Shortcut for search (empty keyword to clear search)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("exit | quit          ", Style::default().fg(Color::White)),
            Span::raw("- Quit the application dashboard"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("ESC                  ", Style::default().fg(Color::White)),
            Span::raw("- Immediate escape"),
        ]),
    ];
    let help_box = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Command Help Area ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(help_box, chunks[4]);
}
