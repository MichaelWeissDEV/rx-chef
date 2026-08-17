//! One module per top-level command.
//!
//! Each module owns the behaviour of a single subcommand and nothing else:
//! argument definitions live in [`crate::cli`], input selection in
//! [`crate::input`], result writing in [`crate::output`], and pipeline
//! execution in [`crate::steps`].

pub(crate) mod bake;
pub(crate) mod history;
pub(crate) mod magic;
pub(crate) mod operations;
pub(crate) mod pipe;
pub(crate) mod pipeline;
pub(crate) mod project;
pub(crate) mod recipe;
pub(crate) mod run;
pub(crate) mod scan;
pub(crate) mod serve;
pub(crate) mod var;
