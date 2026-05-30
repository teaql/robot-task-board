use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

fn colorize_comment_segment<'a>(text: &'a str, spans: &mut Vec<Span<'a>>) {
    let mut current_idx = 0;
    while let Some(start) = text[current_idx..].find('(') {
        let abs_start = current_idx + start;
        if let Some(end) = text[abs_start..].find(')') {
            let abs_end = abs_start + end;
            let inner = &text[abs_start + 1..abs_end];
            if !inner.is_empty() && (inner.chars().all(|c| c.is_ascii_digit()) || inner == "pending") {
                if abs_start > current_idx {
                    spans.push(Span::styled(&text[current_idx..abs_start + 1], Style::default().fg(Color::Rgb(230, 126, 34)).add_modifier(Modifier::BOLD)));
                }
                spans.push(Span::styled(inner, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
                current_idx = abs_end;
                continue;
            }
        }
        // If not matching, just skip the '('
        spans.push(Span::styled(&text[current_idx..abs_start + 1], Style::default().fg(Color::Rgb(230, 126, 34)).add_modifier(Modifier::BOLD)));
        current_idx = abs_start + 1;
    }
    if current_idx < text.len() {
        spans.push(Span::styled(&text[current_idx..], Style::default().fg(Color::Rgb(230, 126, 34)).add_modifier(Modifier::BOLD)));
    }
}
