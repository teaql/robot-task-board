//! Generated TeaQL domain crate for `robot-kanban-service`.
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

pub use e::*;
pub use q::*;
pub use request_support::*;
pub use runtime::*;
pub use sample_data::*;
pub use platform::*;
pub use task_status::*;
pub use task::*;
pub use task_execution_log::*;