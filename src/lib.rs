/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Public library entry point for rxchef.
 * -----------------------------------------------------------------------------
 */

pub mod catalog;
pub mod execute;
pub mod execution;
pub mod ffi;
pub mod integration;
pub mod magic;
pub mod operation;
pub mod operations;
pub mod pipeline;
pub mod runtime;
pub mod scan;

pub use operation::{
    ArgKind, ArgSchema, ArgValue, DataType, InputRequirement, Operation, OperationData,
    OperationError, OperationStatus, ParityStatus, SideEffect,
};
pub use pipeline::{Pipeline, PipelineError};
