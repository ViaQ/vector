//! Hyper TLS support via OpenSSL.
#![warn(missing_docs)]

use hyper::rt::{Read, ReadBuf, ReadBufCursor, Write};
use openssl::error::ErrorStack;
use openssl::ssl::{self, ErrorCode, Ssl, SslRef};
use std::fmt;
use std::future;
use std::io::{self, Write as _};
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll};

#[cfg(feature = "client-legacy")]
pub mod client;
#[cfg(test)]
mod test;

struct StreamWrapper<S> {
    stream: S,
    context: *mut Context<'static>,
}

unsafe impl<S> Sync for StreamWrapper<S> where S: Sync {}
unsafe impl<S> Send for StreamWrapper<S> where S: Send {}

impl<S> fmt::Debug for StreamWrapper<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.stream, fmt)
    }
}

impl<S> StreamWrapper<S> {
    /// # Safety
    ///
    /// Must be called with `context` set to a valid pointer to a live `Context` object, and the
    /// wrapper must be pinned in memory.
    unsafe fn parts(&mut self) -> (Pin<&mut S>, &mut Context<'_>) {
        debug_assert_ne!(self.context, ptr::null_mut());
        let stream = Pin::new_unchecked(&mut self.stream);
        let context = &mut *self.context.cast();
        (stream, context)
    }
}

impl<S> io::Read for StreamWrapper<S>
where
    S: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let (stream, cx) = unsafe { self.parts() };
        let mut buf = ReadBuf::new(buf);
        match stream.poll_read(cx, buf.unfilled())? {
            Poll::Ready(()) => Ok(buf.filled().len()),
            Poll::Pending => Err(io::Error::from(io::ErrorKind::WouldBlock)),
        }
    }
}

impl<S> io::Write for StreamWrapper<S>
where
    S: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let (stream, cx) = unsafe { self.parts() };
        match stream.poll_write(cx, buf) {
            Poll::Ready(r) => r,
            Poll::Pending => Err(io::Error::from(io::ErrorKind::WouldBlock)),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let (stream, cx) = unsafe { self.parts() };
        match stream.poll_flush(cx) {
            Poll::Ready(r) => r,
            Poll::Pending => Err(io::Error::from(io::ErrorKind::WouldBlock)),
        }
    }
}

/// A Hyper stream type using OpenSSL.
#[derive(Debug)]
pub struct SslStream<S>(ssl::SslStream<StreamWrapper<S>>);

impl<S> SslStream<S>
where
    S: Read + Write,
{
    /// Like [`SslStream::new`](ssl::SslStream::new).
    pub fn new(ssl: Ssl, stream: S) -> Result<Self, ErrorStack> {
        ssl::SslStream::new(
            ssl,
            StreamWrapper {
                stream,
                context: ptr::null_mut(),
            },
        )
        .map(SslStream)
    }

    /// Like [`SslStream::connect`](ssl::SslStream::connect).
    pub fn poll_connect(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), ssl::Error>> {
        self.with_context(cx, |s| cvt_ossl(s.connect()))
    }

    /// A convenience method wrapping [`poll_connect`](Self::poll_connect).
    pub async fn connect(mut self: Pin<&mut Self>) -> Result<(), ssl::Error> {
        future::poll_fn(|cx| self.as_mut().poll_connect(cx)).await
    }

    /// Like [`SslStream::accept`](ssl::SslStream::accept).
    pub fn poll_accept(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), ssl::Error>> {
        self.with_context(cx, |s| cvt_ossl(s.accept()))
    }

    /// A convenience method wrapping [`poll_accept`](Self::poll_accept).
    pub async fn accept(mut self: Pin<&mut Self>) -> Result<(), ssl::Error> {
        future::poll_fn(|cx| self.as_mut().poll_accept(cx)).await
    }

    /// Like [`SslStream::do_handshake`](ssl::SslStream::do_handshake).
    pub fn poll_do_handshake(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), ssl::Error>> {
        self.with_context(cx, |s| cvt_ossl(s.do_handshake()))
    }

    /// A convenience method wrapping [`poll_do_handshake`](Self::poll_do_handshake).
    pub async fn do_handshake(mut self: Pin<&mut Self>) -> Result<(), ssl::Error> {
        future::poll_fn(|cx| self.as_mut().poll_do_handshake(cx)).await
    }
}

impl<S> SslStream<S> {
    /// Returns a shared reference to the `Ssl` object associated with this stream.
    pub fn ssl(&self) -> &SslRef {
        self.0.ssl()
    }

    /// Returns a shared reference to the underlying stream.
    pub fn get_ref(&self) -> &S {
        &self.0.get_ref().stream
    }

    /// Returns a mutable reference to the underlying stream.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.0.get_mut().stream
    }

    /// Returns a pinned mutable reference to the underlying stream.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut S> {
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0.get_mut().stream) }
    }

    fn with_context<F, R>(self: Pin<&mut Self>, ctx: &mut Context<'_>, f: F) -> R
    where
        F: FnOnce(&mut ssl::SslStream<StreamWrapper<S>>) -> R,
    {
        unsafe {
            let this = self.get_unchecked_mut();
            this.0.get_mut().context = (ctx as *mut Context<'_>).cast();
            let r = f(&mut this.0);
            this.0.get_mut().context = ptr::null_mut();
            r
        }
    }
}

impl<S> Read for SslStream<S>
where
    S: Read + Write,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        // SAFETY: read_uninit doesn't de-initialize the buffer and guarantees that nread bytes have
        // been initialized.
        self.with_context(cx, |s| unsafe {
            match cvt(s.read_uninit(buf.as_mut()))? {
                Poll::Ready(nread) => {
                    buf.advance(nread);
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => Poll::Pending,
            }
        })
    }
}

impl<S> Write for SslStream<S>
where
    S: Read + Write,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.with_context(cx, |s| cvt(s.write(buf)))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.with_context(cx, |s| cvt(s.flush()))
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.with_context(cx, |s| {
            match s.shutdown() {
                Ok(_) => {}
                Err(e) => match e.code() {
                    ErrorCode::ZERO_RETURN => {}
                    ErrorCode::WANT_READ | ErrorCode::WANT_WRITE => return Poll::Pending,
                    _ => {
                        return Poll::Ready(Err(e
                            .into_io_error()
                            .unwrap_or_else(|e| io::Error::new(io::ErrorKind::Other, e))))
                    }
                },
            }

            let (stream, cx) = unsafe { s.get_mut().parts() };
            stream.poll_shutdown(cx)
        })
    }
}

fn cvt<T>(r: io::Result<T>) -> Poll<io::Result<T>> {
    match r {
        Ok(v) => Poll::Ready(Ok(v)),
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => Poll::Pending,
        Err(e) => Poll::Ready(Err(e)),
    }
}

fn cvt_ossl<T>(r: Result<T, ssl::Error>) -> Poll<Result<T, ssl::Error>> {
    match r {
        Ok(v) => Poll::Ready(Ok(v)),
        Err(e) => match e.code() {
            ErrorCode::WANT_READ | ErrorCode::WANT_WRITE => Poll::Pending,
            _ => Poll::Ready(Err(e)),
        },
    }
}
