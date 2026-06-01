#[tokio::main]
async fn main() {
    let service = robot_task_board::service::TaskService::new("test_facet.db").await.unwrap();
    let query = robot_kanban_service::Q::tasks()
        .facet_by_status_as("status_stats", robot_kanban_service::Q::task_status().count_tasks());
    let res = query.execute_for_list(&service.ctx).await;
    println!("Res: {:?}", res.is_ok());
}
