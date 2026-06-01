import re

with open('src/service.rs', 'r') as f:
    content = f.read()

test_code = """
    #[tokio::test]
    async fn test_facet_and_count() -> Result<(), Box<dyn Error>> {
        let db_path = "test_facet_and_count.db";
        let _ = std::fs::remove_file(db_path);

        let service = TaskService::new(db_path).await?;
        
        // Add tasks to a specific status (Planned usually gets tasks initially)
        let t1 = service.add_task("T1").await?;
        let t2 = service.add_task("T2").await?;
        service.move_task(t1, "Ready").await?;

        // Query statuses and verify counts using facet_by
        let tasks = robot_kanban::Q::tasks()
            .facet_by_status_as("status_stats", robot_kanban::Q::task_status().count_tasks())
            .execute_for_list(&service.ctx)
            .await?;

        let mut ready_count = 0;
        let mut planned_count = 0;

        let statuses = tasks.facets.get("status_stats").unwrap();
        for status in &statuses.data {
            let count = match status.get("count_tasks") {
                Some(teaql_core::Value::I64(c)) => *c,
                _ => 0,
            };
            let name = match status.get("name") {
                Some(teaql_core::Value::Text(s)) => s.as_str(),
                _ => "",
            };
            if name == "Ready" {
                ready_count = count;
                assert_eq!(count, 1);
            }
            if name == "Planned" {
                planned_count = count;
                assert_eq!(count, 1);
            }
        }
        
        assert_eq!(ready_count, 1);
        assert_eq!(planned_count, 1);

        Ok(())
    }
"""

content = content.replace("    #[tokio::test]", test_code + "\n    #[tokio::test]")

with open('src/service.rs', 'w') as f:
    f.write(content)
