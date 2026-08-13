//! Convenience entry points for the shared execution engine.

pub use crate::execution::{
    ExecutionError, ExecutionOptions, ExecutionOutcome, ExecutionRequest, Recipe, RecipeStep,
    VariableContext,
};

/// Execute one operation with raw input bytes.
///
/// # Errors
///
/// Returns a structured [`ExecutionError`] for lookup, argument, operation, or
/// resource-limit failures.
///
/// # Examples
///
/// ```
/// let outcome = rxchef::execute::run("To Base64", b"hi".to_vec(), vec![])?;
/// assert_eq!(outcome.output, b"aGk=");
/// # Ok::<(), rxchef::execute::ExecutionError>(())
/// ```
pub fn run(
    operation: impl Into<String>,
    input: Vec<u8>,
    args: Vec<String>,
) -> Result<ExecutionOutcome, ExecutionError> {
    crate::execution::run(operation, input, args)
}

/// Execute a complete recipe using the supplied input, variables, and limits.
///
/// # Errors
///
/// Returns a structured [`ExecutionError`] if validation, execution, or a
/// configured resource limit fails.
pub fn bake(request: ExecutionRequest) -> Result<ExecutionOutcome, ExecutionError> {
    crate::execution::execute(request)
}
