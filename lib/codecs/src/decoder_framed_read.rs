use bytes::BytesMut;
use futures_core::Stream;
use pin_project::pin_project;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

/// Internal wrapper that converts decoder errors into `Ok(Some(Err(..)))`.
struct DecoderResultWrapper<D> {
    inner: D,
}

impl<D> DecoderResultWrapper<D>
where
    D: Decoder,
{
    const fn new(inner: D) -> Self {
        Self { inner }
    }
}

impl<D> Decoder for DecoderResultWrapper<D>
where
    D: Decoder,
{
    type Item = Result<D::Item, D::Error>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.inner.decode(src) {
            Ok(item) => Ok(item.map(Ok)),
            Err(error) => Ok(Some(Err(error))),
        }
    }

    fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.inner.decode_eof(src) {
            Ok(item) => Ok(item.map(Ok)),
            Err(error) => Ok(Some(Err(error))),
        }
    }
}

/// A `FramedRead` wrapper that continues decoding after recoverable decoder errors.
///
/// Standard `FramedRead` terminates the stream when a decoder returns an error.
/// This wrapper converts decoder errors into successful results so the stream continues,
/// allowing callers to decide whether to continue via `StreamDecodingError::can_continue()`.
#[pin_project]
pub struct DecoderFramedRead<T, D> {
    #[pin]
    inner: FramedRead<T, DecoderResultWrapper<D>>,
}

impl<T, D> DecoderFramedRead<T, D>
where
    T: AsyncRead,
    D: Decoder,
{
    /// Creates a new `DecoderFramedRead` with the default buffer capacity.
    pub fn new(inner: T, decoder: D) -> Self {
        Self {
            inner: FramedRead::new(inner, DecoderResultWrapper::new(decoder)),
        }
    }

    /// Creates a new `DecoderFramedRead` with the given buffer capacity.
    pub fn with_capacity(inner: T, decoder: D, capacity: usize) -> Self {
        Self {
            inner: FramedRead::with_capacity(inner, DecoderResultWrapper::new(decoder), capacity),
        }
    }

    /// Returns a reference to the underlying I/O stream.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying I/O stream.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Returns a reference to the read buffer.
    pub fn read_buffer(&self) -> &BytesMut {
        self.inner.read_buffer()
    }
}

impl<T, D> Stream for DecoderFramedRead<T, D>
where
    T: AsyncRead,
    D: Decoder,
    D::Error: From<io::Error>,
{
    type Item = Result<D::Item, D::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match this.inner.poll_next(cx) {
            Poll::Ready(Some(Ok(Ok(item)))) => Poll::Ready(Some(Ok(item))),
            Poll::Ready(Some(Ok(Err(error)))) => Poll::Ready(Some(Err(error))),
            Poll::Ready(Some(Err(error))) => Poll::Ready(Some(Err(error.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
