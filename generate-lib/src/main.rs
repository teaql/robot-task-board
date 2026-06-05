#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _runtime = robot_kanban_service::service_runtime_from_env().await?;

    // Uncomment the following line to generate sample data for testing:
    // robot_kanban_service::sample_data::generate_sample_data(&_runtime, robot_kanban_service::sample_data::SampleDataPlan::small()).await?;
    Ok(())
}