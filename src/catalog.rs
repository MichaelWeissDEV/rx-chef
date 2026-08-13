//! Stable public access to the operation catalog.

pub use crate::integration::{ArgumentDescriptor, OperationDescriptor};

/// Errors returned by catalog lookup.
#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    /// No registered operation matched the supplied name or normalized ID.
    #[error("operation not found: {0}")]
    NotFound(String),
    /// Registered metadata could not be materialized.
    #[error("invalid catalog metadata: {0}")]
    InvalidMetadata(String),
}

/// Return all registered operation descriptors in stable name order.
///
/// # Errors
///
/// Returns [`CatalogError::InvalidMetadata`] if a registry entry cannot be
/// represented as a public descriptor.
pub fn operations() -> Result<Vec<OperationDescriptor>, CatalogError> {
    crate::integration::operations().map_err(CatalogError::InvalidMetadata)
}

/// Resolve a display name or normalized identifier and return its descriptor.
///
/// # Errors
///
/// Returns [`CatalogError::NotFound`] when no operation matches.
///
/// # Examples
///
/// ```
/// let descriptor = rxchef::catalog::describe("from_base64")?;
/// assert_eq!(descriptor.name, "From Base64");
/// # Ok::<(), rxchef::catalog::CatalogError>(())
/// ```
pub fn describe(operation: &str) -> Result<OperationDescriptor, CatalogError> {
    crate::integration::describe(operation)
        .map_err(|_| CatalogError::NotFound(operation.to_string()))
}
