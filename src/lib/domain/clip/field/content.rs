use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

/// Represents the content of a clip.
///
/// # Errors
///
/// Returns a `ClipError::EmptyContent` if the provided content is empty or consists only of whitespace.
///
/// # Examples
///
/// ```
/// use clip_stash::domain::clip::field::content::Content;
/// use clip_stash::domain::clip::ClipError;
///
/// let content = Content::new("Some content").unwrap();
/// assert_eq!(content.as_str(), "Some content");
///
/// let empty_content = Content::new("");
/// assert!(matches!(empty_content, Err(ClipError::EmptyContent)));
/// ```
impl Content {
    /// Creates a new `Content` instance.
    ///
    /// # Arguments
    ///
    /// * `content` - A string slice that holds the content.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the content is not empty or whitespace.
    /// * `Err(ClipError::EmptyContent)` - If the content is empty or consists only of whitespace.
    pub fn new(content: &str) -> Result<Self, ClipError> {
        // Implementation
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    /// Consumes the `Content` and returns the inner string.
    ///
    /// # Returns
    ///
    /// * `String` - The inner string content.
    pub fn into_inner(self) -> String {
        // Implementation
        self.0
    }

    /// Returns a reference to the inner string content.
    ///
    /// # Returns
    ///
    /// * `&str` - A reference to the inner string content.
    pub fn as_str(&self) -> &str {
        // Implementation
        self.0.as_str()
    }
}
