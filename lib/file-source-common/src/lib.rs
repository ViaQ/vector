#![deny(warnings)]
#![deny(clippy::all)]

use std::borrow::Cow;

pub mod buffer;
pub mod checkpointer;
mod fingerprinter;
pub mod internal_events;
mod metadata_ext;

use vector_config::configurable_component;

/// The marker appended to a message that has been shortened to fit a size limit, so that
/// downstream consumers can tell the message is incomplete.
pub const TRUNCATION_MARKER: &str = "..TRUNCATED";

/// Shortens `bytes` so that it ends with [`TRUNCATION_MARKER`] and occupies at most
/// `max_bytes` in total, without ever splitting a UTF-8 character.
///
/// Unlike [`truncate_bytes_for_size`], the marker is appended even if `bytes`
/// is smaller than `max_bytes`. This is intended for data that was already
/// cut short by a reader, ensuring downstream consumers know it is incomplete.
pub fn mark_truncated_bytes(bytes: &[u8], max_bytes: usize) -> Vec<u8> {
    if max_bytes < TRUNCATION_MARKER.len() {
        // Not enough room for the marker at all: hard-truncate at nearest UTF-8 boundary.
        let mut len = max_bytes.min(bytes.len());
        while len > 0 && !is_utf8_boundary(bytes, len) {
            len -= 1;
        }
        return bytes[..len].to_vec();
    }
    let mut len = (max_bytes - TRUNCATION_MARKER.len()).min(bytes.len());
    while len > 0 && !is_utf8_boundary(bytes, len) {
        len -= 1;
    }
    let mut result = Vec::with_capacity(len + TRUNCATION_MARKER.len());
    result.extend_from_slice(&bytes[..len]);
    result.extend_from_slice(TRUNCATION_MARKER.as_bytes());
    result
}

/// Truncates bytes to at most `max_bytes` bytes, appending `b"..TRUNCATED"`, without ever
/// splitting a UTF-8 character. Works directly on byte slices for efficiency.
pub fn truncate_bytes_for_size(bytes: &[u8], max_bytes: usize) -> Vec<u8> {
    const TRUNCATED_SUFFIX: &[u8] = TRUNCATION_MARKER.as_bytes();

    if bytes.len() <= max_bytes {
        return bytes.to_vec();
    }
    if max_bytes < TRUNCATED_SUFFIX.len() {
        // Not enough room for the suffix at all: hard-truncate at nearest UTF-8 boundary.
        let mut len = max_bytes.min(bytes.len());
        while len > 0 && !is_utf8_boundary(bytes, len) {
            len -= 1;
        }
        return bytes[..len].to_vec();
    }
    let mut len = max_bytes - TRUNCATED_SUFFIX.len();
    while len > 0 && !is_utf8_boundary(bytes, len) {
        len -= 1;
    }
    let mut result = bytes[..len].to_vec();
    result.extend_from_slice(TRUNCATED_SUFFIX);
    result
}

/// Helper: checks if `pos` is a valid UTF-8 boundary in `bytes`.
#[inline]
fn is_utf8_boundary(bytes: &[u8], pos: usize) -> bool {
    if pos == 0 || pos == bytes.len() {
        return true;
    }
    if pos > bytes.len() {
        return false;
    }
    // A valid UTF-8 boundary is where the byte is not a continuation byte (0b10xxxxxx).
    (bytes[pos] & 0b11000000) != 0b10000000
}

/// Truncates a string to at most `max_bytes` bytes, appending `..TRUNCATED` (matching the
/// convention used for merged oversized lines in `kubernetes_logs`), without ever splitting
/// a UTF-8 character.
pub fn truncate_for_size(s: &str, max_bytes: usize) -> String {
    String::from_utf8_lossy(&truncate_bytes_for_size(s.as_bytes(), max_bytes)).into_owned()
}

/// Legacy truncation function with `[...]` suffix, kept for backward compatibility with
/// parser error reporting. New code should use [`truncate_for_size`] with `..TRUNCATED` suffix.
pub fn truncate_string_at(s: &str, maxlen: usize) -> Cow<'_, str> {
    let ellipsis: &str = "[...]";
    if s.len() <= maxlen {
        return s.into();
    }
    if maxlen < ellipsis.len() {
        let mut len = maxlen;
        while !s.is_char_boundary(len) {
            len -= 1;
        }
        return s[..len].to_string().into();
    }
    let mut len = maxlen - ellipsis.len();
    while !s.is_char_boundary(len) {
        len -= 1;
    }
    format!("{}{}", &s[..len], ellipsis).into()
}

pub use self::{
    checkpointer::{CHECKPOINT_FILE_NAME, Checkpointer, CheckpointsView},
    fingerprinter::{FileFingerprint, FingerprintStrategy, Fingerprinter},
    internal_events::FileSourceInternalEvents,
    metadata_ext::{AsyncFileInfo, PortableFileExt},
};

pub type FilePosition = u64;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ReadFrom {
    #[default]
    Beginning,
    End,
    Checkpoint(FilePosition),
}

/// File position to use when reading a new file.
#[configurable_component]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReadFromConfig {
    /// Read from the beginning of the file.
    Beginning,

    /// Start reading from the current end of the file.
    End,
}

impl From<ReadFromConfig> for ReadFrom {
    fn from(rfc: ReadFromConfig) -> Self {
        match rfc {
            ReadFromConfig::Beginning => ReadFrom::Beginning,
            ReadFromConfig::End => ReadFrom::End,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_utf8_boundary() {
        let message = "Hello 😁 this is test.";
        let result = truncate_for_size(message, 18);
        assert!(result.ends_with("..TRUNCATED"));
    }

    #[test]
    fn truncate_no_truncation_needed() {
        let message = "Short";
        assert_eq!("Short", truncate_for_size(message, 20));
    }

    #[test]
    fn truncate_exact_length() {
        let message = "Hello";
        assert_eq!("Hello", truncate_for_size(message, 5));
    }

    #[test]
    fn truncate_multibyte_char_at_boundary() {
        let message = "Hello😁World This is extra padding";
        // max_bytes = 18.
        // 18 - 11 (suffix) = 7.
        // Byte 7 falls exactly inside the 4-byte emoji (which occupies bytes 5, 6, 7, 8).
        // The function will snap back to the nearest boundary at byte 5 ("Hello").
        let result = truncate_for_size(message, 18);

        assert!(result.ends_with("..TRUNCATED"));
        assert_eq!(result, "Hello..TRUNCATED");
    }

    #[test]
    fn truncate_empty_string() {
        let message = "";
        assert_eq!("", truncate_for_size(message, 10));
    }

    #[test]
    fn truncate_with_emoji_only() {
        let message = "😁😁😁😁😁"; // 20 bytes
        // 16 - 11 = 5. Byte 5 falls inside the second emoji, snapping back to byte 4.
        let result = truncate_for_size(message, 16);
        assert_eq!(result, "😁..TRUNCATED");
    }

    #[test]
    fn truncate_maxlen_shorter_than_suffix_does_not_panic() {
        // `max_bytes` smaller than "..TRUNCATED" (11 bytes) must not underflow
        // `max_bytes - TRUNCATED_SUFFIX.len()`. The result must still respect `max_bytes`.
        let message = "Hello this is a long line";
        let result = truncate_for_size(message, 3);
        assert!(
            result.len() <= 3,
            "result was {} bytes: {result:?}",
            result.len()
        );
    }

    #[test]
    fn truncate_maxlen_one_byte_does_not_panic() {
        let message = "Hello this is a long line";
        let result = truncate_for_size(message, 1);
        assert!(
            result.len() <= 1,
            "result was {} bytes: {result:?}",
            result.len()
        );
    }

    #[test]
    fn truncate_maxlen_zero_does_not_panic() {
        let message = "Hello this is a long line";
        let result = truncate_for_size(message, 0);
        assert_eq!("", result);
    }

    // Tests for legacy truncate_string_at (with [...] suffix)
    #[test]
    fn truncate_string_at_utf8_boundary() {
        let message = "Hello 😁 this is test.";
        assert_eq!("Hello [...]", truncate_string_at(message, 13));
    }

    #[test]
    fn truncate_string_at_no_truncation_needed() {
        let message = "Short";
        assert_eq!("Short", truncate_string_at(message, 20).as_ref());
    }

    #[test]
    fn truncate_string_at_exact_length() {
        let message = "Hello";
        assert_eq!("Hello", truncate_string_at(message, 5).as_ref());
    }

    #[test]
    fn truncate_string_at_multibyte_char_at_boundary() {
        let message = "Hello😁World";
        let result = truncate_string_at(message, 8);
        assert!(result.ends_with("[...]"));
    }

    #[test]
    fn truncate_string_at_empty_string() {
        let message = "";
        assert_eq!("", truncate_string_at(message, 10).as_ref());
    }

    #[test]
    fn truncate_string_at_with_emoji_only() {
        let message = "😁😁😁";
        let result = truncate_string_at(message, 5);
        assert!(result.ends_with("[...]"));
    }

    #[test]
    fn truncate_string_at_maxlen_shorter_than_ellipsis_does_not_panic() {
        let message = "Hello this is a long line";
        let result = truncate_string_at(message, 3);
        assert!(
            result.len() <= 3,
            "result was {} bytes: {result:?}",
            result.len()
        );
    }

    #[test]
    fn truncate_string_at_maxlen_one_byte_does_not_panic() {
        let message = "Hello this is a long line";
        let result = truncate_string_at(message, 1);
        assert!(
            result.len() <= 1,
            "result was {} bytes: {result:?}",
            result.len()
        );
    }

    #[test]
    fn truncate_string_at_maxlen_zero_does_not_panic() {
        let message = "Hello this is a long line";
        let result = truncate_string_at(message, 0);
        assert_eq!("", result.as_ref());
    }

    // Tests for truncate_bytes_for_size
    #[test]
    fn truncate_bytes_for_size_basic() {
        let bytes = b"Hello this is a long line";
        let result = truncate_bytes_for_size(bytes, 15);
        assert_eq!(result.len(), 15);
        assert!(result.ends_with(b"..TRUNCATED"));
    }

    #[test]
    fn truncate_bytes_for_size_no_truncation_needed() {
        let bytes = b"Short";
        let result = truncate_bytes_for_size(bytes, 20);
        assert_eq!(result, bytes);
    }

    #[test]
    fn truncate_bytes_for_size_utf8_boundary() {
        let message = "Hello 😁 this is test.";
        let bytes = message.as_bytes();
        let result = truncate_bytes_for_size(bytes, 13);
        // Should not panic and should respect UTF-8 boundaries
        assert!(result.len() <= 13);
        assert!(result.ends_with(b"..TRUNCATED"));
        // Ensure result is valid UTF-8
        let _ = String::from_utf8(result).expect("result should be valid UTF-8");
    }

    #[test]
    fn truncate_bytes_for_size_multibyte_char_at_boundary() {
        let message = "Hello😁World";
        let bytes = message.as_bytes();
        let result = truncate_bytes_for_size(bytes, 8);
        assert!(result.len() <= 8);
        // Should be valid UTF-8 after truncation
        let _ = String::from_utf8(result).expect("result should be valid UTF-8");
    }

    #[test]
    fn truncate_bytes_for_size_maxlen_shorter_than_suffix() {
        let message = "Hello this is a long line";
        let bytes = message.as_bytes();
        let result = truncate_bytes_for_size(bytes, 3);
        assert!(
            result.len() <= 3,
            "result was {} bytes: {:?}",
            result.len(),
            String::from_utf8_lossy(&result)
        );
    }

    #[test]
    fn truncate_bytes_for_size_with_invalid_utf8() {
        // Invalid UTF-8 sequence: 0xFF is not valid
        let bytes: &[u8] = &[
            b'H', b'e', b'l', b'l', b'o', 0xFF, b'W', b'o', b'r', b'l', b'd',
        ];
        let result = truncate_bytes_for_size(bytes, 8);
        assert!(result.len() <= 8);
        // The truncation should avoid splitting at the invalid byte
    }

    #[test]
    fn truncate_bytes_for_size_zero_max() {
        let bytes = b"Hello";
        let result = truncate_bytes_for_size(bytes, 0);
        assert_eq!(result.len(), 0);
    }
}
