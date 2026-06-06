with open('/home/philip/githome/teaql-code-gen/generator/src/main/resources/generator/stacks/rust/expression/index.stg', 'r') as f:
    content = f.read()

old_block = """            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                let access_path = if attempted_path.starts_with(failed_node.as_str()) { let mut p = attempted_path.clone(); p.insert_str(failed_node.len(), "\\<break\\>"); p } else { format!("{}\\<break\\>", attempted_path) }; panic!("\\n\\n💥 [Coding Logic Bug] Attempted to evaluate an unloaded field or relation!\\n\\nRoot Object : {}\\nAccess Path : {}\\n\\nHint: You forgot to explicitly request this data from the database.\\nPlease ensure you select it in your query.\\n", self.root_desc, access_path);
            }"""

new_block = """            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                let mut formatted_path = String::new();
                let mut break_inserted = false;
                for part in attempted_path.split('.') {
                    let formatted_part = format!(".get_{}()", part);
                    formatted_path.push_str(&formatted_part);
                    if part == failed_node && !break_inserted {
                        formatted_path.push_str("\\\\<break\\\\>");
                        break_inserted = true;
                    }
                }
                panic!("\\n\\n💥 [Coding Logic Bug] {}{}, please use select_{}() to load data.\\n", self.root_desc, formatted_path, failed_node);
            }"""

content = content.replace(old_block, new_block)

with open('/home/philip/githome/teaql-code-gen/generator/src/main/resources/generator/stacks/rust/expression/index.stg', 'w') as f:
    f.write(content)
