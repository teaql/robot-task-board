//! #  Service Workspace
//!
//! **Before writing queries**, read these generated documents:
//! - [`build/API_GUIDE.md`](./build/API_GUIDE.md) — domain API (entities, filters, relations, E expressions)
//! - [`build/TOOL_API.md`](./build/TOOL_API.md) — framework runtime (UserContext, SmartList, Save pipeline)

pub use robot_kanban_service::{teaql_core, E, Q};

pub fn generated_domain_crate() -> &'static str {
    "robot-kanban-service"
}