//! Error types and handling for keypath operations

use thiserror::Error;

/// Errors that can occur during keypath operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum KeyPathError {
    /// Keypath access failed due to invalid path
    #[error("Invalid keypath access: {message}")]
    InvalidAccess { message: String },
    
    /// Type mismatch during keypath operation
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch { expected: String, found: String },
    
    /// Runtime failure during keypath operation
    #[error("Runtime failure: {message}")]
    RuntimeFailure { message: String },
    
    /// Collection operation failed
    #[error("Collection operation failed: {message}")]
    CollectionError { message: String },
    
    /// Async operation failed
    #[error("Async operation failed: {message}")]
    AsyncError { message: String },
    
    /// Parallel operation failed
    #[error("Parallel operation failed: {message}")]
    ParallelError { message: String },
}

/// Result type for keypath operations
pub type KeyPathResult<T> = Result<T, KeyPathError>;

/// Extension trait for converting standard errors to KeyPathError
pub trait IntoKeyPathError {
    fn into_keypath_error(self) -> KeyPathError;
}

impl<E: std::error::Error> IntoKeyPathError for E {
    fn into_keypath_error(self) -> KeyPathError {
        KeyPathError::RuntimeFailure {
            message: self.to_string(),
        }
    }
}

/// Helper macros for error creation
#[macro_export]
macro_rules! keypath_error {
    ($variant:ident, $($field:ident: $value:expr),*) => {
        KeyPathError::$variant {
            $($field: $value),*
        }
    };
}

#[macro_export]
macro_rules! keypath_result {
    ($expr:expr) => {
        $expr.map_err(|e| e.into_keypath_error())
    };
}

/// Validation utilities for keypath operations
pub mod validation {
    use super::*;
    
    /// Validate that a keypath operation is safe to perform
    pub fn validate_keypath_access<T>(_data: &T) -> KeyPathResult<()> {
        // This is a placeholder for more sophisticated validation
        // In a real implementation, this might check for null pointers,
        // bounds checking, etc.
        Ok(())
    }
    
    /// Validate that a collection operation is safe to perform
    pub fn validate_collection_operation<T>(collection: &[T]) -> KeyPathResult<()> {
        if collection.is_empty() {
            return Err(KeyPathError::CollectionError {
                message: "Collection is empty".to_string(),
            });
        }
        Ok(())
    }
}
