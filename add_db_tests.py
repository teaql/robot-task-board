import re

with open('src/service.rs', 'r') as f:
    content = f.read()

test_code = """
    #[tokio::test]
    async fn test_database_initialization() -> Result<(), Box<dyn Error>> {
        let db_path = "test_db_init.db";
        let _ = std::fs::remove_file(db_path);

        // 1. First open: should create tables and seed data
        let service = TaskService::new(db_path).await?;
        assert!(!service.status_map.is_empty(), "Status map should be populated on first initialization");
        
        // 2. Second open: should reuse tables and still load status
        let service2 = TaskService::new(db_path).await?;
        assert!(!service2.status_map.is_empty(), "Status map should be populated on second initialization");
        
        // Ensure same data is accessible
        let task_id = service2.add_task("Test Init Task").await?;
        
        // 3. Third open: verify task exists
        let service3 = TaskService::new(db_path).await?;
        let tasks = robot_kanban::Q::tasks().execute_for_list(&service3.ctx).await?.data;
        let found = tasks.iter().any(|t| t.id() == task_id);
        assert!(found, "Task created in session 2 should be visible in session 3");

        Ok(())
    }

    #[tokio::test]
    async fn test_database_concurrent_open() -> Result<(), Box<dyn Error>> {
        let db_path = "test_db_concurrent.db";
        let _ = std::fs::remove_file(db_path);

        let service1 = TaskService::new(db_path).await?;
        let service2 = TaskService::new(db_path).await?;

        let t1 = service1.add_task("T1").await?;
        let t2 = service2.add_task("T2").await?;
        
        assert_ne!(t1, t2);

        Ok(())
    }
"""

if "test_database_initialization" not in content:
    content = content.replace("    #[tokio::test]\n    async fn test_core_flow", test_code + "\n    #[tokio::test]\n    async fn test_core_flow")

    with open('src/service.rs', 'w') as f:
        f.write(content)
