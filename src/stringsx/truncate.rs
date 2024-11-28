//! Provides string truncation utilities.

use std::str;

/// Truncates a string to a specified byte length while maintaining UTF-8 validity.
///
/// This function ensures that the returned string is valid UTF-8 by finding the largest
/// valid UTF-8 boundary that fits within the specified byte length.
///
/// # Arguments
///
/// * `s` - The input string to truncate
/// * `length` - The maximum byte length to truncate to
///
/// # Returns
///
/// A string slice containing the truncated string. If the input is already shorter than
/// the specified length or if length is 0, returns the original string unchanged.
///
/// # Examples
///
/// ```
/// use crate::toolkit::stringsx::truncate::truncate_byte_len;
///
/// assert_eq!(truncate_byte_len("Hello, World", 5), "Hello");
/// assert_eq!(truncate_byte_len("Hello,🚧", 7), "Hello,");
/// ```
pub fn truncate_byte_len(s: &str, length: usize) -> &str {
    if length == 0 || s.len() < length {
        return s;
    }

    let mut valid_len = length;
    while str::from_utf8(&s.as_bytes()[..valid_len]).is_err() {
        valid_len -= 1;
    }

    &s[..valid_len]
}

#[cfg(test)]
mod tests {
    use crate::stringsx::truncate::truncate_byte_len;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate_byte_len("Test", 10), "Test");
        assert_eq!(truncate_byte_len("Hello, World", 5), "Hello");
        assert_eq!(truncate_byte_len("Hello,🚧", 7), "Hello,");
    }
}
