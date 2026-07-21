use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::io::AsyncRead as Read;
use http_types::Body;
use pin_project::pin_project;

use crate::chunked::ChunkedEncoder;

#[pin_project(project=BodyEncoderProjection)]
#[derive(Debug)]
pub(crate) enum BodyEncoder {
    Chunked(#[pin] ChunkedEncoder<Body>),
    Fixed(#[pin] Body),
}

impl BodyEncoder {
    pub(crate) fn new(body: Body) -> Self { panic!("STUB: not implemented") }
}

impl Read for BodyEncoder {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> { panic!("STUB: not implemented") }
}
