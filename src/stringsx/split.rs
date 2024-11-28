//! String splitting utility functions.
//!
//! This module provides functions for splitting strings into vectors
//! of substrings using separators.

/// Splits a string slice into a vector of string slices using a separator
///
/// # Arguments
///
/// * `s` - The string slice to split
/// * `sep` - The separator string to split on
///
/// # Returns
///
/// A vector of string slices containing the split components
///
/// # Examples
///
/// ```
/// use crate::toolkit::stringsx::split::splitx;
///
/// let result = splitx("hello world", " ");
/// assert_eq!(result, vec!["hello", "world"]);
///
/// let empty = splitx("", ",");
/// assert_eq!(empty, Vec::<&str>::new());
/// ```
pub fn splitx<'word>(s: &'word str, sep: &'_ str) -> Vec<&'word str> {
    // Return empty vector if input string is empty
    if s.is_empty() {
        return vec![];
    }
    // Split string on separator and collect into vector
    s.split(sep).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the splitx function with various inputs
    #[test]
    fn test_split() {
        assert_eq!(splitx("hello world", " "), vec!["hello", "world"]);
        assert_eq!(splitx("", ""), Vec::<&str>::new());
    }
}
