use crate::chunked::ChunkedDecoder;
use async_dup::{Arc, Mutex};
use futures_lite::io::{AsyncRead as Read, BufReader, Take};
use std::{
    fmt::Debug,
    io,
    pin::Pin,
    task::{Context, Poll},
};

pub enum BodyReader<IO: Read + Unpin> {
    Chunked(Arc<Mutex<ChunkedDecoder<BufReader<IO>>>>),
    Fixed(Arc<Mutex<Take<BufReader<IO>>>>),
    None,
}

impl<IO: Read + Unpin> Debug for BodyReader<IO> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}

impl<IO: Read + Unpin> Read for BodyReader<IO> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> { panic!("STUB: not implemented") }
}
