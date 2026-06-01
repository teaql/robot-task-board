use robot_kanban_service::runtime::TeaqlRepositoryProvider;
use robot_task_board::service::KanbanService;

#[tokio::main]
async fn main() {
    let service = KanbanService::new().unwrap();
    let query = robot_kanban_service::entities::Q::tasks()
        .facet_by_status_as("status_stats", robot_kanban_service::entities::Q::task_status().count_tasks());
    
    let list_result = query.execute_for_list(&service.ctx).await.unwrap();
    if let Some(facet_list) = list_result.facet("status_stats") {
        for record in facet_list.iter() {
            println!("Facet Record: {:?}", record);
        }
    } else {
        println!("No facet returned");
    }
}
