//! Convenience wrapper for streams to switch between plain TCP and TLS at runtime.
//!
//!  There is no dependency on actual TLS implementations. Everything like
//! `native_tls` or `openssl` will work as long as there is a TLS stream supporting standard
//! `Read + Write` traits.
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// A stream that might be protected with TLS.
#[non_exhaustive]
#[pin_project(project = StreamProj)]
#[derive(Debug)]
pub enum MaybeTlsStream<S> {
    /// Unencrypted socket stream.
    Plain(#[pin] S),
    /// Encrypted socket stream using `native-tls`.
    #[cfg(feature = "native-tls")]
    NativeTls(tokio_native_tls::TlsStream<S>),
    /// Encrypted socket stream using `rustls`.
    #[cfg(feature = "rustls-tls")]
    Rustls(tokio_rustls::client::TlsStream<S>),
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for MaybeTlsStream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.project() {
            StreamProj::Plain(ref mut s) => Pin::new(s).poll_read(cx, buf),
            #[cfg(feature = "native-tls")]
            StreamProj::NativeTls(s) => Pin::new(s).poll_read(cx, buf),
            #[cfg(feature = "rustls-tls")]
            StreamProj::Rustls(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeTlsStream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match self.project() {
            StreamProj::Plain(ref mut s) => Pin::new(s).poll_write(cx, buf),
            #[cfg(feature = "native-tls")]
            StreamProj::NativeTls(s) => Pin::new(s).poll_write(cx, buf),
            #[cfg(feature = "rustls-tls")]
            StreamProj::Rustls(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        match self.project() {
            StreamProj::Plain(ref mut s) => Pin::new(s).poll_flush(cx),
            #[cfg(feature = "native-tls")]
            StreamProj::NativeTls(s) => Pin::new(s).poll_flush(cx),
            #[cfg(feature = "rustls-tls")]
            StreamProj::Rustls(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match self.project() {
            StreamProj::Plain(ref mut s) => Pin::new(s).poll_shutdown(cx),
            #[cfg(feature = "native-tls")]
            StreamProj::NativeTls(s) => Pin::new(s).poll_shutdown(cx),
            #[cfg(feature = "rustls-tls")]
            StreamProj::Rustls(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}
