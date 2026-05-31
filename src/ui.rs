use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::app::App;

pub fn parse_log_line(line: &str) -> Line<'_> {
    let mut spans = Vec::new();
    let mut rest = line;

    // Detect if this is an AUDIT line (set during level bracket parsing)
    let mut is_audit = false;

    // Detect aligned format: e.g. [08:32:31.456]-[user]-[DEBUG/AUDIT]-message
    if line.starts_with('[') && line.len() > 15 {
        if let Some(time_end) = line.find(']') {
            let timestamp = &line[1..time_end];
            // Only match if the bracket contents look like a time (contains colon)
            if timestamp.contains(':') {
                spans.push(Span::styled(format!("[{}]", timestamp), Style::default().fg(Color::Indexed(244))));
                rest = &line[time_end+1..];

                // 2. User ID bracket e.g. -[user]
                if rest.starts_with("-[") {
                    if let Some(end) = rest[2..].find(']') {
                        let user_part = &rest[2..end+2];
                        spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                        spans.push(Span::styled(format!("[{}]", user_part), Style::default().fg(Color::Rgb(155, 89, 182)).add_modifier(Modifier::BOLD)));
                        rest = &rest[end+3..];
                    }
                }

                // 3. Severity Level bracket e.g. -[AUDIT], -[INFO], or -[DEBUG]
                if rest.starts_with("-[") {
                    if let Some(end) = rest[2..].find(']') {
                        let level = &rest[2..end+2];
                        spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                        if level == "AUDIT" {
                            is_audit = true;
                            spans.push(Span::styled(format!("[{}]", level), Style::default().fg(Color::Rgb(230, 126, 34)).add_modifier(Modifier::BOLD)));
                        } else if level == "INFO" {
                            spans.push(Span::styled(format!("[{}]", level), Style::default().fg(Color::Rgb(46, 204, 113)).add_modifier(Modifier::BOLD)));
                        } else {
                            spans.push(Span::styled(format!("[{}]", level), Style::default().fg(Color::Indexed(242))));
                        }
                        rest = &rest[end+3..];
                    }
                }

                if rest.starts_with('-') {
                    spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                    rest = &rest[1..];
                }
            } else {
                // Fallback for indented lines or others
                spans.push(Span::styled(line, Style::default().fg(Color::White)));
                return Line::from(spans);
            }
        }
    } else {
        // Fallback for other lines
        spans.push(Span::styled(line, Style::default().fg(Color::White)));
        return Line::from(spans);
    }

    // Now highlight the rest of the message!
    // Handle extra bracket after timing (e.g. [1234µs] or [DEBUG] in reformatted SQL logs)
    if rest.starts_with("[") {
        if let Some(end) = rest[1..].find(']') {
            let tag = &rest[1..end+1];
            // Highlight µs timing in red, other tags in gray
            let tag_style = if tag.ends_with("µs") {
                Style::default().fg(Color::Rgb(231, 76, 60)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(242))
            };
            spans.push(Span::styled(format!("[{}]", tag), tag_style));
            rest = &rest[end+2..];
            if rest.starts_with('-') {
                spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                rest = &rest[1..];
            }
        }
    }

    // Parse the next bracket too (e.g. [DEBUG] after [µs])
    if rest.starts_with("[") {
        if let Some(end) = rest[1..].find(']') {
            let tag = &rest[1..end+1];
            let tag_style = if tag.ends_with("µs") {
                Style::default().fg(Color::Rgb(231, 76, 60)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(242))
            };
            spans.push(Span::styled(format!("[{}]", tag), tag_style));
            rest = &rest[end+2..];
            if rest.starts_with('-') {
                spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                rest = &rest[1..];
            }
        }
    }

    if rest.starts_with("SqlLogEntry") {
        spans.push(Span::styled("SqlLogEntry", Style::default().fg(Color::Indexed(242))));
        rest = &rest[11..];
    }

    // Use AUDIT orange for the entire message body if this is an AUDIT line
    if is_audit {
        colorize_comment_segment(rest, &mut spans, Style::default().fg(Color::Rgb(230, 126, 34)));
        return Line::from(spans);
    }

    // 4. Comment part and Result summary part
    if rest.starts_with(" - [") {
        if let Some(end) = rest[4..].find(']') {
            let first_segment = &rest[..end+5];
            let after_first = &rest[end+5..];
            
            if after_first.starts_with(" - [") {
                // If there is another " - [" immediately following, then the first one is the comment!
                colorize_comment_segment(first_segment, &mut spans, Style::default().fg(Color::Rgb(230, 126, 34)).add_modifier(Modifier::BOLD));
                rest = after_first;
                
                // Now parse the second one as the result summary
                if let Some(end2) = rest[4..].find(']') {
                    let result_part = &rest[..end2+5];
                    colorize_comment_segment(result_part, &mut spans, Style::default().fg(Color::Rgb(52, 152, 219)));
                    rest = &rest[end2+5..];
                }
            } else {
                // If there is no " - [" following, then this first segment is the result summary (no comment exists)!
                colorize_comment_segment(first_segment, &mut spans, Style::default().fg(Color::Rgb(52, 152, 219)));
                rest = after_first;
            }
        }
    }

    // 5. Highlight Changes in Audit logs or remaining SQL
    if let Some(changes_idx) = rest.find(" Changes: ") {
        let main_msg = &rest[..changes_idx];
        let changes = &rest[changes_idx..];
        spans.push(Span::styled(main_msg, Style::default().fg(Color::White)));
        spans.push(Span::styled(changes, Style::default().fg(Color::Cyan)));
    } else {
        colorize_sql(rest, &mut spans);
    }

    Line::from(spans)
}

fn colorize_comment_segment<'a>(text: &'a str, spans: &mut Vec<Span<'a>>, base_style: Style) {
    let mut current_idx = 0;
    while let Some(start) = text[current_idx..].find('(') {
        let abs_start = current_idx + start;
        if let Some(end) = text[abs_start..].find(')') {
            let abs_end = abs_start + end;
            let inner = &text[abs_start + 1..abs_end];
            if !inner.is_empty() && (inner.chars().all(|c| c.is_ascii_digit()) || inner == "pending") {
                if abs_start > current_idx {
                    spans.push(Span::styled(&text[current_idx..abs_start + 1], base_style));
                }
                spans.push(Span::styled(inner, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
                current_idx = abs_end;
                continue;
            }
        }
        // If not matching, just skip the '('
        spans.push(Span::styled(&text[current_idx..abs_start + 1], base_style));
        current_idx = abs_start + 1;
    }
    if current_idx < text.len() {
        spans.push(Span::styled(&text[current_idx..], base_style));
    }
}

fn colorize_sql<'a>(sql: &'a str, spans: &mut Vec<Span<'a>>) {
    let mut current_idx = 0;
    let mut text_start = 0;

    while let Some(quote_idx) = sql[current_idx..].find('\'') {
        let abs_quote = current_idx + quote_idx;
        if abs_quote > text_start {
            colorize_sql_text(&sql[text_start..abs_quote], spans);
        }
        
        let mut end_idx = abs_quote + 1;
        loop {
            if let Some(next_quote) = sql[end_idx..].find('\'') {
                end_idx += next_quote; // this is the index of the next quote
                if end_idx + 1 < sql.len() && sql[end_idx + 1..].starts_with('\'') {
                    end_idx += 2; // skip escaped quote
                } else {
                    end_idx += 1; // include the closing quote
                    break;
                }
            } else {
                end_idx = sql.len();
                break;
            }
        }
        spans.push(Span::styled(sql[abs_quote..end_idx].to_owned(), Style::default().fg(Color::Red)));
        current_idx = end_idx;
        text_start = current_idx;
    }
    
    if text_start < sql.len() {
        colorize_sql_text(&sql[text_start..], spans);
    }
}

fn colorize_sql_text<'a>(text: &'a str, spans: &mut Vec<Span<'a>>) {
    let mut in_word = false;
    let mut word_start = 0;
    
    for (i, c) in text.char_indices() {
        let is_ident = c.is_alphanumeric() || c == '_' || c == '.';
        
        if !in_word && is_ident {
            if i > word_start {
                spans.push(Span::styled(text[word_start..i].to_owned(), Style::default().fg(Color::DarkGray)));
            }
            word_start = i;
            in_word = true;
        } else if in_word && !is_ident {
            let word = &text[word_start..i];
            let is_param = (word != "." && word.chars().all(|ch| ch.is_ascii_digit() || ch == '.'))
                        || word.eq_ignore_ascii_case("true") 
                        || word.eq_ignore_ascii_case("false") 
                        || word.eq_ignore_ascii_case("null");
            let color = if is_param { Color::Red } else { Color::DarkGray };
            spans.push(Span::styled(word.to_owned(), Style::default().fg(color)));
            word_start = i;
            in_word = false;
        }
    }
    
    if word_start < text.len() {
        if in_word {
            let word = &text[word_start..];
            let is_param = (word != "." && word.chars().all(|ch| ch.is_ascii_digit() || ch == '.'))
                        || word.eq_ignore_ascii_case("true") 
                        || word.eq_ignore_ascii_case("false") 
                        || word.eq_ignore_ascii_case("null");
            let color = if is_param { Color::Red } else { Color::DarkGray };
            spans.push(Span::styled(word.to_owned(), Style::default().fg(color)));
        } else {
            spans.push(Span::styled(text[word_start..].to_owned(), Style::default().fg(Color::DarkGray)));
        }
    }
}

pub fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if app.hide_logs {
            vec![
                Constraint::Length(3),      // 1. System Info area
                Constraint::Length(3),      // 2. Status statistics
                Constraint::Min(5),         // 3. Columns (Planned, Process, Done)
                Constraint::Length(3),      // 4. Command Line Area
                Constraint::Length(3),      // 5. Command Help Area
            ]
        } else {
            vec![
                Constraint::Percentage(60), // 1. Log area
                Constraint::Length(3),      // 2. Status statistics
                Constraint::Min(5),         // 3. Columns (Planned, Process, Done)
                Constraint::Length(3),      // 4. Command Line Area
                Constraint::Length(3),      // 5. Command Help Area
            ]
        })
        .split(f.size());

    // 1. Render Log Area (Keeps beautiful syntax-highlighted logs)
    let log_height = chunks[0].height as usize - 2; // Subtract borders
    let total_logs = app.logs.len();
    let max_scroll = total_logs.saturating_sub(log_height);
    let scroll_offset = app.log_scroll_offset.min(max_scroll);

    let skip_count = total_logs.saturating_sub(log_height + scroll_offset);
    let log_lines: Vec<Line> = app.logs.iter()
        .skip(skip_count)
        .take(log_height)
        .map(|l| parse_log_line(l))
        .collect();

    let search_str = if let Some(ref term) = app.search_term {
        term.as_str()
    } else {
        "None"
    };

    let scroll_str = if scroll_offset == 0 {
        "Bottom".to_owned()
    } else {
        format!("Up {}/{}", scroll_offset, max_scroll)
    };

    let title_str = if app.hide_logs {
        format!(" System Info | CPU: {} | MEM: {} | Search: {} ", app.cpu_model, app.mem_size, search_str)
    } else {
        format!(" TeaQL Business Trace & SQL Introspection | Scroll: {} | CPU: {} | MEM: {} | Search: {} ", scroll_str, app.cpu_model, app.mem_size, search_str)
    };

    let log_paragraph = Paragraph::new(if app.hide_logs { vec![] } else { log_lines })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title_str)
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::Indexed(240))),
    );
    f.render_widget(log_paragraph, chunks[0]);

    // 2. Render Status Statistics Area (4 equal columns)
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    let planned_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Planned: "),
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

    let ready_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Ready: "),
        Span::styled(
            format!("{}", app.ready_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(ready_stat, stats_chunks[1]);

    let executing_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Executing: "),
        Span::styled(
            format!("{}", app.executing_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(executing_stat, stats_chunks[2]);

    let verified_stat = Paragraph::new(Line::from(vec![
        Span::raw("  Verified: "),
        Span::styled(
            format!("{}", app.verified_count),
            Style::default().fg(Color::White),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(verified_stat, stats_chunks[3]);

    // 3. Render task list columns (4 columns)
    let col_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[2]);

    // Planned Tasks column
    let planned_lines = app
        .planned_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::LightGreen)),
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

    // Ready Tasks column
    let ready_lines = app
        .ready_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::LightGreen)),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let ready_list = Paragraph::new(ready_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" READY ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(ready_list, col_chunks[1]);

    // Executing Tasks column
    let executing_lines = app
        .executing_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::LightGreen)),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let executing_list = Paragraph::new(executing_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" EXECUTING ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(executing_list, col_chunks[2]);

    // Verified Tasks column
    let verified_lines = app
        .verified_tasks
        .iter()
        .map(|t| Line::from(vec![
            Span::styled(format!("  {:>4}  ", t.id()), Style::default().fg(Color::LightGreen)),
            Span::styled(t.name().to_string(), Style::default().fg(Color::White)),
        ]))
        .collect::<Vec<Line>>();
    let verified_list = Paragraph::new(verified_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" VERIFIED ")
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(verified_list, col_chunks[3]);

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
            Span::styled("<name>", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" Add  "),
            Span::styled("/mv <id> [status]", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" Move  "),
            Span::styled("/del <id>", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" Delete  "),
            Span::styled("/s <kw>", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" Search  "),
            Span::styled("/q", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("|"),
            Span::styled("ESC", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" Quit  "),
            Span::styled("↑↓/PgUp/PgDn", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw(" Scroll logs"),
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
