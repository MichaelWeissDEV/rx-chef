//! Process-level error type and its human-readable rendering.
//!
//! Library code returns structured errors; this is the only place that turns
//! them into CLI text and process exit codes.

use rxchef::{execution, runtime};

#[derive(Debug)]
pub(crate) enum CliError {
    InvalidInput(String),
    Execution(String),
    StoreIo(String),
    FeatureUnavailable(String),
}

impl CliError {
    pub(crate) fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidInput(_) => 3,
            Self::Execution(_) => 4,
            Self::StoreIo(_) => 5,
            Self::FeatureUnavailable(_) => 6,
        }
    }
}

impl From<execution::ExecutionError> for CliError {
    fn from(error: execution::ExecutionError) -> Self {
        match &error {
            execution::ExecutionError::RuntimeStep {
                source: runtime::RuntimeError::Unavailable { .. },
                ..
            } => Self::FeatureUnavailable(error.to_string()),
            _ => Self::Execution(error.to_string()),
        }
    }
}

impl From<rxchef::operation::OperationError> for CliError {
    fn from(error: rxchef::operation::OperationError) -> Self {
        match error {
            rxchef::operation::OperationError::InvalidInput(message) => Self::InvalidInput(message),
            rxchef::operation::OperationError::InvalidArgument { name, reason } => {
                Self::InvalidInput(format!("invalid argument '{name}': {reason}"))
            }
            rxchef::operation::OperationError::ProcessingError(message) => Self::Execution(message),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message)
            | Self::Execution(message)
            | Self::StoreIo(message)
            | Self::FeatureUnavailable(message) => formatter.write_str(message),
        }
    }
}
