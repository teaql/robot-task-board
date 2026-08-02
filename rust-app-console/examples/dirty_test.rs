use robot_kanban::*;
use teaql_core::Entity;

fn main() {
    // Create a fresh task via from_record (simulates loading from DB)
    let mut record = teaql_core::Record::new();
    record.insert("id".to_owned(), teaql_core::Value::U64(1));
    record.insert("name".to_owned(), teaql_core::Value::Text("Test".to_owned()));
    record.insert("version".to_owned(), teaql_core::Value::I64(1));
    record.insert("status_id".to_owned(), teaql_core::Value::U64(1001));
    record.insert("platform_id".to_owned(), teaql_core::Value::U64(1));

    let mut task = Task::from_record(record).expect("from_record");

    // Before any update, dirty_fields should be None
    println!("Before update: dirty_fields = {:?}", task.dirty_fields());

    // Update only status
    task.update_status_to_ready();

    // After update, dirty_fields should be Some({"status_id"})
    println!("After update_status_to_ready: dirty_fields = {:?}", task.dirty_fields());
}
