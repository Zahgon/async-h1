use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::io::{self, AsyncRead as Read, Cursor};
use http_types::headers::{CONTENT_LENGTH, HOST, TRANSFER_ENCODING};
use http_types::{Method, Request};

use crate::body_encoder::BodyEncoder;
use crate::read_to_end;
use crate::EncoderState;

#[derive(Debug)]
pub struct Encoder {
    request: Request,
    state: EncoderState,
}

impl Encoder {
    
    pub fn new(request: Request) -> Self { panic!("STUB: not implemented") }

    fn finalize_headers(&mut self) -> io::Result<()> { panic!("STUB: not implemented") }

    fn compute_head(&mut self) -> io::Result<Cursor<Vec<u8>>> { panic!("STUB: not implemented") }
}

impl Read for Encoder {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> { panic!("STUB: not implemented") }
}
