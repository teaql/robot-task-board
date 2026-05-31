use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style, Modifier};

fn main() {
    let mut spans: Vec<Span> = Vec::new();
    let rest = "Execute TeaQL - Q::tasks().comment(\"Get task for DDD\").with_id_is(1)";
    if rest.starts_with("Execute TeaQL - ") {
        spans.push(Span::styled("Execute TeaQL - ", Style::default().fg(Color::Indexed(242))));
        spans.push(Span::styled(&rest[16..], Style::default().fg(Color::Rgb(46, 204, 113)).add_modifier(Modifier::BOLD)));
    }
    println!("{:?}", spans);
}
