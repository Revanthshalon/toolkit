//! This module provides string case manipulation utilities.
//!
//! It contains functions for manipulating the case of strings, specifically:
//! - Converting first character to lowercase
//! - Converting first character to uppercase
//!
//! # Performance Notes
//! The functions in this module are optimized for performance but may be further
//! optimized in future versions.
//!
//! # Examples
//! ```
//! use crate::toolkit::stringsx::case::{to_lower_initials, to_upper_initials};
//!
//! assert_eq!(to_lower_initials("Hello"), "hello");
//! assert_eq!(to_upper_initials("world"), "World");
//! ```

/// Converts the first character of a string to lowercase and returns the modified string
///
/// # Arguments
///
/// * `s` - A string slice that will have its first character converted to lowercase
///
/// # Returns
///
/// * A new String with the first character in lowercase
///
/// # Examples
///
/// ```
/// use crate::toolkit::stringsx::case::to_lower_initials;
///
/// let result = to_lower_initials("Hello");
/// assert_eq!(result, "hello");
/// ```
pub fn to_lower_initials(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    // WARN: This function is optimized for performance. Optimize this later on by directly
    // changing the first letter case, in the string as is.
    let mut letters = s.chars();
    let initial = letters.next().unwrap().to_lowercase().to_string();
    let remaining = letters.collect::<String>();
    format!("{}{}", initial, remaining)
}

/// Converts the first character of a string to uppercase and returns the modified string
///
/// # Arguments
///
/// * `s` - A string slice that will have its first character converted to uppercase
///
/// # Returns
///
/// * A new String with the first character in uppercase
///
/// # Examples
///
/// ```
/// use crate::toolkit::stringsx::case::to_upper_initials;
///
/// let result = to_upper_initials("hello");
/// assert_eq!(result, "Hello");
/// ```
pub fn to_upper_initials(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    // WARN: This function is optimized for performance. Optimize this later on by directly
    // changing the first letter case, in the string as is.
    let mut letters = s.chars();
    let initial = letters.next().unwrap().to_uppercase().to_string();
    let remaining = letters.collect::<String>();
    format!("{}{}", initial, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lower_initials() {
        assert_eq!(to_lower_initials("Hello"), "hello");
        assert_eq!(to_lower_initials("World"), "world");
        assert_eq!(to_lower_initials("world"), "world");
        assert_eq!(to_lower_initials(""), "");
    }

    #[test]
    fn test_to_upper_initials() {
        assert_eq!(to_upper_initials("hello"), "Hello");
        assert_eq!(to_upper_initials("world"), "World");
        assert_eq!(to_upper_initials("WORLD"), "WORLD");
        assert_eq!(to_upper_initials(""), "");
    }
}
