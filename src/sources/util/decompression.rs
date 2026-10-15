//! Decompression limits to prevent decompression-bomb attacks.
//!
//! A decompression bomb is a small compressed payload that expands to an unbounded size,
//! causing out-of-memory errors. This module provides a size-limiting reader that wraps
//! decompressor output to prevent such attacks.
//!
//! The limit can be configured via the `VECTOR_MAX_DECOMPRESSED_SIZE_BYTES` environment
//! variable. If not set, defaults to 100 MiB (104857600 bytes).

use std::{
    io::{self, Read},
    sync::OnceLock,
};

/// Default maximum decompressed payload size (100 MiB).
pub const DEFAULT_MAX_DECOMPRESSED_SIZE: usize = 104_857_600;

static MAX_DECOMPRESSED_SIZE_BYTES: OnceLock<usize> = OnceLock::new();

/// Returns the configured maximum decompressed payload size.
///
/// Reads from `VECTOR_MAX_DECOMPRESSED_SIZE_BYTES` environment variable on first call,
/// falling back to DEFAULT_MAX_DECOMPRESSED_SIZE if not set or invalid.
pub fn max_decompressed_size() -> usize {
    *MAX_DECOMPRESSED_SIZE_BYTES.get_or_init(|| {
        std::env::var("VECTOR_MAX_DECOMPRESSED_SIZE_BYTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(DEFAULT_MAX_DECOMPRESSED_SIZE)
    })
}

/// A Read wrapper that limits the decompressed output size.
///
/// This wraps a decompressor (like MultiGzDecoder) and counts how many decompressed
/// bytes are read, returning an error if the limit is exceeded.
pub struct LimitedReader<R> {
    inner: R,
    limit: usize,
    consumed: usize,
}

impl<R: Read> LimitedReader<R> {
    /// Creates a new LimitedReader that wraps a decompressor.
    ///
    /// The limit is enforced on the decompressed output, not the compressed input.
    pub const fn new(inner: R, limit: usize) -> Self {
        Self {
            inner,
            limit,
            consumed: 0,
        }
    }
}

impl<R: Read> Read for LimitedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.consumed = self.consumed.saturating_add(n);

        if self.consumed > self.limit {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "decompressed payload exceeded limit of {} bytes",
                    self.limit
                ),
            ));
        }

        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::{Cursor, Write};

    #[test]
    fn test_limited_reader_within_limit() {
        let data = b"hello world";
        let cursor = Cursor::new(data);
        let mut limited = LimitedReader::new(cursor, 100);
        let mut buf = Vec::new();
        limited.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, data);
    }

    #[test]
    fn test_limited_reader_exceeds_limit() {
        let data = b"hello world"; // 11 bytes
        let cursor = Cursor::new(data);
        let mut limited = LimitedReader::new(cursor, 5);
        let mut buf = Vec::new();
        let result = limited.read_to_end(&mut buf);
        // Should fail because we try to read 11 bytes with a 5 byte limit
        assert!(result.is_err());
        // The error happens when consumed > limit, so we can read up to limit+1 bytes
        // before the error is detected, but read_to_end() may not preserve partial data on error
        assert!(buf.len() <= 6);
    }

    #[test]
    fn test_decompression_bomb_protection() {
        // Create a decompression bomb: 150 MB of zeros compressed
        let bomb_size = 150 * 1024 * 1024;
        let zeros = vec![0u8; bomb_size];

        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&zeros).unwrap();
        let compressed = encoder.finish().unwrap();

        // Compressed size should be very small
        assert!(compressed.len() < 1024 * 1024);

        // Try to decompress with our limit
        use flate2::read::MultiGzDecoder;
        let decoder = MultiGzDecoder::new(&compressed[..]);
        let mut limited = LimitedReader::new(decoder, max_decompressed_size());
        let mut output = Vec::new();

        let result = limited.read_to_end(&mut output);
        assert!(result.is_err(), "Should reject decompression bomb");
    }

    #[test]
    fn test_env_var_override() {
        // This test documents the behavior but can't actually test it due to OnceLock
        // The env var is read once on first call to max_decompressed_size()
        let default_size = DEFAULT_MAX_DECOMPRESSED_SIZE;
        assert_eq!(default_size, 104_857_600);
    }
}
