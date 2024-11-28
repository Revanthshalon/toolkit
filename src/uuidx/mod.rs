//! UUID Generation Utilities
//!
//! This module provides functionality for generating UUIDs (Universally Unique Identifiers).
//! Currently supports generating random version 4 UUIDs.

use uuid::Uuid;

/// Generates a new random UUID v4
///
/// # Returns
///
/// A randomly generated version 4 UUID
///
/// # Example
///
/// ```
/// use crate::toolkit::uuidx::new_v4;
///
/// let uuid = new_v4();
/// ```
pub fn new_v4() -> Uuid {
    Uuid::new_v4()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_v4() {
        let uuid = new_v4();
        assert_eq!(uuid.get_version_num(), 4);
    }
}
