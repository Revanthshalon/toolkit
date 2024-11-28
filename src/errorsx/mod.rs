//! Enhanced error handling module providing structured error types with context
//!
//! This module provides the [`ErrorX`] type and [`ErrorXBuilder`] for creating rich error
//! objects that capture:
//!
//! - Error messages
//! - Stack backtraces
//! - Source code locations
//! - Contextual information
//! - Original source errors
//! - Status codes and messages
//!
//! # Example
//! ```
//! # use std::io;
//! # use crate::toolkit::errorsx::{ErrorX, ErrorXBuilder};
//!
//! let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
//! let err = ErrorX::builder("Failed to process file")
//!     .with_context("Processing user upload")
//!     .with_source(io_error)
//!     .with_status_code(500)
//!     .build();
//! ```

use std::{backtrace::Backtrace, error::Error, fmt::Display, panic::Location};

/// A structured error type that contains message, backtrace, location and context information
///
/// # Fields
/// * `message` - The error message string
/// * `backtrace` - The stack backtrace when error occurred
/// * `location` - The source code location where error was created
/// * `context` - Vector of contextual information strings
/// * `source` - Optional underlying error that caused this error
/// * `status_code` - Optional HTTP status code
/// * `status` - Optional status message string
#[derive(Debug)]
pub struct ErrorX {
    message: String,
    backtrace: Backtrace,
    location: &'static Location<'static>,
    context: Vec<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
    status_code: Option<u32>,
    status: Option<String>,
}

impl Display for ErrorX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let context_info = self.context.join(",");
        let location_info = format!(
            "(at: {}, line_no:{})",
            self.location.file(),
            self.location.line()
        );
        write!(
            f,
            "Message:{},\nLocation: {},\nContext: {},\nSource:\n {:#?}",
            self.message, location_info, context_info, self.backtrace
        )
    }
}

impl Error for ErrorX {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

/// Builder for constructing ErrorX instances with a fluent API
///
/// # Fields
/// * `message` - The error message string
/// * `context` - Vector of contextual information strings
/// * `location` - The source code location where builder was created
/// * `source` - Optional underlying error that caused this error
/// * `status_code` - Optional HTTP status code
/// * `status` - Optional status message string
#[derive(Debug)]
pub struct ErrorXBuilder {
    message: String,
    context: Vec<String>,
    location: &'static Location<'static>,
    source: Option<Box<dyn Error + Send + Sync>>,
    status_code: Option<u32>,
    status: Option<String>,
}

impl ErrorXBuilder {
    /// Initializes a new ErrorXBuilder with the given message
    ///
    /// # Parameters
    /// * `message` - The error message to set
    ///
    /// # Returns
    /// * `Self` - A new ErrorXBuilder instance
    #[track_caller]
    pub fn init(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: Vec::<String>::new(),
            location: Location::caller(),
            source: None,
            status_code: None,
            status: None,
        }
    }

    /// Adds context information to the error
    ///
    /// # Parameters
    /// * `context` - The context string to add
    ///
    /// # Returns
    /// * `Self` - The builder instance for chaining
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }

    /// Sets the source error that caused this error
    ///
    /// # Parameters
    /// * `source` - The source error to set
    ///
    /// # Returns
    /// * `Self` - The builder instance for chaining
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Sets an HTTP-style status code for the error
    ///
    /// # Parameters
    /// * `status_code` - The status code to set
    ///
    /// # Returns
    /// * `Self` - The builder instance for chaining
    pub fn with_status_code(mut self, status_code: u32) -> Self {
        self.status_code = Some(status_code);
        self
    }

    /// Sets a status string for the error
    ///
    /// # Parameters
    /// * `status` - The status string to set
    ///
    /// # Returns
    /// * `Self` - The builder instance for chaining
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Builds and returns the ErrorX instance
    ///
    /// # Returns
    /// * `ErrorX` - The constructed error instance
    pub fn build(self) -> ErrorX {
        ErrorX {
            message: self.message,
            context: self.context,
            location: self.location,
            backtrace: Backtrace::force_capture(),
            source: self.source,
            status_code: self.status_code,
            status: self.status,
        }
    }
}

impl ErrorX {
    /// Creates a new ErrorX with just a message
    ///
    /// # Parameters
    /// * `message` - The error message
    ///
    /// # Returns
    /// * `Self` - A new ErrorX instance
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        ErrorXBuilder::init(message).build()
    }

    /// Returns a builder for constructing an ErrorX with more options
    ///
    /// # Parameters
    /// * `message` - The initial error message
    ///
    /// # Returns
    /// * `ErrorXBuilder` - A new builder instance
    #[track_caller]
    pub fn builder(message: impl Into<String>) -> ErrorXBuilder {
        ErrorXBuilder::init(message)
    }

    /// Returns the error message
    ///
    /// # Returns
    /// * `&str` - Reference to the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the context information
    ///
    /// # Returns
    /// * `&Vec<String>` - Reference to the context vector
    pub fn context(&self) -> &Vec<String> {
        &self.context
    }

    /// Returns the location where the error was created
    ///
    /// # Returns
    /// * `&Location` - Reference to the source location
    pub fn location(&self) -> &Location<'_> {
        self.location
    }

    /// Returns the error backtrace
    ///
    /// # Returns
    /// * `&Backtrace` - Reference to the backtrace
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    /// Returns the status code if set
    ///
    /// # Returns
    /// * `&Option<u32>` - Reference to the optional status code
    pub fn status_code(&self) -> &Option<u32> {
        &self.status_code
    }

    /// Returns the status string if set
    ///
    /// # Returns
    /// * `&Option<String>` - Reference to the optional status string
    pub fn status(&self) -> &Option<String> {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_errorx_new() {
        let err = ErrorX::new("Test Error");
        assert_eq!(err.message(), "Test Error");
        assert!(err.context().is_empty());
        assert!(err.source().is_none());
        assert!(err.status_code().is_none());
        assert!(err.status().is_none());
    }

    #[test]
    fn test_errorx_builder() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let err = ErrorX::builder("Failed to process file")
            .with_context("Processing user upload")
            .with_source(io_error)
            .with_status_code(500)
            .with_status("Internal Server Error")
            .build();
        let location = err.location();
        let backtrace = err.backtrace();

        assert_eq!(err.message(), "Failed to process file");
        assert_eq!(err.context(), &vec!["Processing user upload".to_string()]);
        assert!(err.source().is_some());
        assert_eq!(location.file(), "src/errorsx/mod.rs");
        assert!(backtrace.to_string().contains("Backtrace"));
        assert_eq!(err.status_code(), &Some(500));
        assert_eq!(err.status(), &Some("Internal Server Error".to_string()));
    }

    #[test]
    fn test_errorsx_display() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let err = ErrorX::builder("Failed to process file")
            .with_context("Processing user upload")
            .with_source(io_error)
            .with_status_code(500)
            .with_status("Internal Server Error")
            .build();
        let err_string = format!("{}", err);
        assert!(err_string.contains("Failed to process file"));
        assert!(err_string.contains("errorsx/mod.rs"));
        assert!(err_string.contains("Context:"));
        assert!(err_string.contains("Source"));
    }

    #[test]
    fn test_errorx_source() {
        let io_error = io::Error::new(io::ErrorKind::Other, "IO Error");
        let err = ErrorX::builder("Higher Level Error")
            .with_source(io_error)
            .build();

        assert!(err.source().is_some());
        assert_eq!(err.source().unwrap().to_string(), "IO Error");
    }
}
