fn main() {
    let sql = "UPDATE \"Task\" SET \"name\" = '中文' WHERE \"id\" = 1";
    let mut spans = Vec::new();
    colorize_sql(sql, &mut spans);
    println!("{:?}", spans);
}

fn colorize_sql<'a>(sql: &'a str, spans: &mut Vec<String>) {
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
        spans.push(format!("RED: {}", &sql[abs_quote..end_idx]));
        current_idx = end_idx;
        text_start = current_idx;
    }
    
    if text_start < sql.len() {
        colorize_sql_text(&sql[text_start..], spans);
    }
}

fn colorize_sql_text<'a>(text: &'a str, spans: &mut Vec<String>) {
    let mut in_word = false;
    let mut word_start = 0;
    
    for (i, c) in text.char_indices() {
        let is_ident = c.is_alphanumeric() || c == '_' || c == '.';
        
        if !in_word && is_ident {
            if i > word_start {
                spans.push(format!("WHITE: {}", &text[word_start..i]));
            }
            word_start = i;
            in_word = true;
        } else if in_word && !is_ident {
            let word = &text[word_start..i];
            let is_param = (word != "." && word.chars().all(|ch| ch.is_ascii_digit() || ch == '.'))
                        || word.eq_ignore_ascii_case("true") 
                        || word.eq_ignore_ascii_case("false") 
                        || word.eq_ignore_ascii_case("null");
            let color = if is_param { "RED" } else { "WHITE" };
            spans.push(format!("{}: {}", color, word));
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
            let color = if is_param { "RED" } else { "WHITE" };
            spans.push(format!("{}: {}", color, word));
        } else {
            spans.push(format!("WHITE: {}", &text[word_start..]));
        }
    }
}
