use robot_kanban::e::E;
use robot_kanban::Q;

#[test]
#[should_panic(expected = "Logic Bug! You forgot to query the 'platform.platform' relation!")]
fn test_e_wrapper_panic_on_not_loaded() {
    let ctx = robot_kanban::module_with_behaviors_and_checkers().into_context();
    let mut task = Q::tasks().purpose("test").new_entity(&ctx);
    
    // Simulate a partial load state where 'platform' relation is missing
    task.set_load_state(teaql_core::eval::LoadState::Partial(
        vec!["id".to_string(), "name".to_string()].into_iter().collect()
    ));

    // This should panic immediately due to the missing 'platform' relation
    let _platform = E::task(&task).get_platform().unwrap();
}
