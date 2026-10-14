//! Versioned byte-exact DAGs. Domain execution has no transport or UI dependencies.
//! Local cancellation/deadlines are cooperative between nodes; hard interruption
//! of an individual operation requires the server's supervised worker process.
mod execution;
mod model;
mod validation;

pub use execution::{execute, ExecutionControl, GraphResult, NodeResult, NodeStatus, OutputResult};
pub use model::*;
pub use validation::{validate, GraphPlan};
#[cfg(test)]
mod tests;
