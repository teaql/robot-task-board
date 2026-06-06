use std::fs;
use std::path::Path;

fn visit_dirs(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else {
                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let mut files = Vec::new();
    visit_dirs(Path::new("generate-lib/lib/src"), &mut files).unwrap();

    for file in files {
        let content = fs::read_to_string(&file).unwrap();
        
        let content_new = content.replace("NotLoaded { missing_path: ", "NotLoaded { failed_node: ");
        let mut final_content = String::new();
        for line in content_new.lines() {
            if line.contains("failed_node:") && line.contains(".to_string() }") {
                let p1 = line.find("failed_node: \"").unwrap() + 14;
                let p2 = line.find("\".to_string() }").unwrap();
                let field = &line[p1..p2];
                let replacement = format!("failed_node: \"{}\".to_string(), attempted_path: \"{}\".to_string() }}", field, field);
                final_content.push_str(&line.replace(&format!("failed_node: \"{}\".to_string() }}", field), &replacement));
                final_content.push('\n');
            } else {
                final_content.push_str(line);
                final_content.push('\n');
            }
        }
        
        let final_content = final_content.replace(
            "teaql_core::eval::EvalResult::NotLoaded { missing_path } => {",
            "teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {"
        );
        let final_content = final_content.replace(
            "panic!(\"Logic Bug! You forgot to query the '{}' relation!\", missing_path);",
            "let access_path = if attempted_path.starts_with(failed_node.as_str()) { let mut p = attempted_path.clone(); p.insert_str(failed_node.len(), \"<break>\"); p } else { format!(\"{}<break>\", attempted_path) }; panic!(\"\\n\\n💥 [Coding Logic Bug] Attempted to evaluate an unloaded field or relation!\\n\\nRoot Object : {} (Not tracked)\\nAccess Path : {}\\n\\nHint: You forgot to explicitly request this data from the database.\\nPlease ensure you select it in your query.\\n\", \"Unknown\", access_path);"
        );
        let final_content = final_content.replace(
            "panic!(\"Logic Bug! You forgot to query the '{}' field or relation!\", missing_path);",
            "let access_path = if attempted_path.starts_with(failed_node.as_str()) { let mut p = attempted_path.clone(); p.insert_str(failed_node.len(), \"<break>\"); p } else { format!(\"{}<break>\", attempted_path) }; panic!(\"\\n\\n💥 [Coding Logic Bug] Attempted to evaluate an unloaded field or relation!\\n\\nRoot Object : {} (Not tracked)\\nAccess Path : {}\\n\\nHint: You forgot to explicitly request this data from the database.\\nPlease ensure you select it in your query.\\n\", \"Unknown\", access_path);"
        );
        
        fs::write(&file, final_content).unwrap();
    }
}
