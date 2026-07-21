
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::SystemTime;

use futures_lite::io::{self, AsyncRead as Read, Cursor};
use http_types::headers::{CONTENT_LENGTH, DATE, TRANSFER_ENCODING};
use http_types::{Method, Response};

use crate::body_encoder::BodyEncoder;
use crate::date::fmt_http_date;
use crate::read_to_end;
use crate::EncoderState;

#[derive(Debug)]
pub struct Encoder {
    response: Response,
    state: EncoderState,
    method: Method,
}

impl Read for Encoder {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> { panic!("STUB: not implemented") }
}

impl Encoder {
    
    pub fn new(response: Response, method: Method) -> Self { panic!("STUB: not implemented") }

    fn finalize_headers(&mut self) { panic!("STUB: not implemented") }

    fn compute_head(&mut self) -> io::Result<Cursor<Vec<u8>>> { panic!("STUB: not implemented") }
}
