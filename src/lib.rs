/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Public library entry point for rxchef.
 * -----------------------------------------------------------------------------
 */

pub mod ffi;
pub mod magic;
pub mod operation;
pub mod operations;
pub mod pipeline;
pub mod runtime;

pub use operation::{ArgSchema, ArgValue, DataType, Operation, OperationData, OperationError};
pub use pipeline::{Pipeline, PipelineError};
