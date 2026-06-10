//! Generated TeaQL domain crate for `robot-kanban-service`.
//!
//! **Before writing queries**, read these generated documents:
//! - [`API_GUIDE.md`](./API_GUIDE.md) (or `build/API_GUIDE.md` in workspace) — domain API (entities, filters, relations, E expressions)
//! - [`TOOL_API.md`](./TOOL_API.md) (or `build/TOOL_API.md` in workspace) — framework runtime (UserContext, SmartList, Save pipeline)
//!
//! AI coding agents must read this crate's `AGENTS.md` before using generated
//! APIs. If this crate was downloaded from a Cargo registry, locate the
//! unpacked crate source or vendor the dependency, then read `AGENTS.md` from
//! the crate root before writing code against it.

pub mod e;
pub mod q;
pub mod request_support;
pub mod runtime;
pub mod sample_data;
pub mod platform;
pub mod task_status;
pub mod task;
pub mod task_execution_log;

pub use teaql_core;
pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use platform::*;
pub use task_status::*;
pub use task::*;
pub use task_execution_log::*;