import re

with open('src/service.rs', 'r') as f:
    content = f.read()

# remove old test logic entirely
start = content.find("        // Query statuses and verify counts")
end = content.find("        Ok(())\n    }")

if start != -1 and end != -1:
    content = content[:start] + content[end:]

new_test = """        // Query statuses and verify counts
        let list_result = robot_kanban::Q::tasks()
            .facet_by_status_as("status_stats", robot_kanban::Q::task_status().count_tasks())
            .execute_for_list(&service.ctx)
            .await?;

        let mut ready_count = 0;
        let mut planned_count = 0;
        
        if let Some(facet_list) = list_result.facet("status_stats") {
            for record in facet_list.iter() {
                let status_id = match record.get("id") {
                    Some(&teaql_core::Value::U64(id)) => id,
                    Some(&teaql_core::Value::I64(id)) => id as u64,
                    _ => 0,
                };
                let count = match record.get("count_tasks") {
                    Some(&teaql_core::Value::I64(c)) => c,
                    Some(&teaql_core::Value::U64(c)) => c as i64,
                    _ => 0,
                };
                
                if status_id == 1002 {
                    ready_count = count;
                } else if status_id == 1001 {
                    planned_count = count;
                }
            }
        }
        
        assert_eq!(ready_count, 1);
        assert_eq!(planned_count, 1);
"""

content = content.replace("        Ok(())\n    }", new_test + "        Ok(())\n    }")

with open('src/service.rs', 'w') as f:
    f.write(content)
