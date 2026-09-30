use std::io::{self, Read};

use bytes::{Buf, Bytes};
use flate2::read::{MultiGzDecoder, ZlibDecoder};
use snap::raw::Decoder as SnappyDecoder;
use warp::http::StatusCode;

use super::super::decompression::{max_decompressed_size, LimitedReader};
use crate::{common::http::ErrorMessage, internal_events::HttpDecompressError};

/// Decompresses the body based on the Content-Encoding header.
///
/// Supports gzip, deflate, snappy, zstd, and identity (no compression).
/// Caps decompressed output at 100 MiB to prevent decompression bomb attacks.
pub fn decode(header: Option<&str>, mut body: Bytes) -> Result<Bytes, ErrorMessage> {
    let limit = max_decompressed_size();

    // Check compressed body size first to prevent buffering oversized payloads
    if body.len() > limit {
        return Err(ErrorMessage::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("Request body exceeds limit of {} bytes", limit),
        ));
    }

    if let Some(encodings) = header {
        for encoding in encodings.rsplit(',').map(str::trim) {
            body = match encoding {
                "identity" => body,
                "gzip" => {
                    let mut decoded = Vec::new();
                    let decoder = MultiGzDecoder::new(body.reader());
                    let mut limited = LimitedReader::new(decoder, max_decompressed_size());
                    limited
                        .read_to_end(&mut decoded)
                        .map_err(|error| handle_decode_error(encoding, error))?;
                    decoded.into()
                }
                "deflate" => {
                    let mut decoded = Vec::new();
                    let decoder = ZlibDecoder::new(body.reader());
                    let mut limited = LimitedReader::new(decoder, max_decompressed_size());
                    limited
                        .read_to_end(&mut decoded)
                        .map_err(|error| handle_decode_error(encoding, error))?;
                    decoded.into()
                }
                "snappy" => {
                    // Check the declared uncompressed size before allocating
                    let limit = max_decompressed_size();
                    let declared_len = snap::raw::decompress_len(&body).map_err(|error| {
                        handle_decode_error(
                            encoding,
                            io::Error::new(io::ErrorKind::InvalidData, error),
                        )
                    })?;
                    if declared_len > limit {
                        return Err(ErrorMessage::new(
                            StatusCode::PAYLOAD_TOO_LARGE,
                            format!("Decompressed snappy body exceeds limit of {} bytes", limit),
                        ));
                    }
                    let decompressed =
                        SnappyDecoder::new()
                            .decompress_vec(&body)
                            .map_err(|error| {
                                handle_decode_error(
                                    encoding,
                                    io::Error::new(io::ErrorKind::InvalidData, error),
                                )
                            })?;
                    decompressed.into()
                }
                "zstd" => {
                    let mut decoded = Vec::new();
                    let limit = max_decompressed_size();
                    // Create decoder and cap the window to prevent large window allocation.
                    // For HTTP Content-Encoding: zstd, RFC 9659 recommends max 8 MB (log 23).
                    let mut decoder = zstd::stream::read::Decoder::new(body.reader())
                        .map_err(|error| handle_decode_error(encoding, error))?;
                    decoder
                        .window_log_max(23) // 8 MB window max per RFC 9659
                        .map_err(|error| handle_decode_error(encoding, error))?;
                    let mut limited = LimitedReader::new(decoder, limit);
                    limited
                        .read_to_end(&mut decoded)
                        .map_err(|error| handle_decode_error(encoding, error))?;
                    decoded.into()
                }
                encoding => {
                    return Err(ErrorMessage::new(
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        format!("Unsupported encoding {}", encoding),
                    ))
                }
            }
        }
    }

    // Final check for identity/no-encoding case
    let limit = max_decompressed_size();
    if body.len() > limit {
        return Err(ErrorMessage::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("Payload exceeds limit of {} bytes", limit),
        ));
    }

    Ok(body)
}

fn handle_decode_error(encoding: &str, error: impl std::error::Error) -> ErrorMessage {
    emit!(HttpDecompressError {
        encoding,
        error: &error
    });
    ErrorMessage::new(
        StatusCode::UNPROCESSABLE_ENTITY,
        format!("Failed decompressing payload with {} decoder.", encoding),
    )
}
