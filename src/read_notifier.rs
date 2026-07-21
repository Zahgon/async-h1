use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_channel::Sender;
use futures_lite::io::{self, AsyncBufRead as BufRead, AsyncRead as Read};

#[pin_project::pin_project]
pub(crate) struct ReadNotifier<B> {
    #[pin]
    reader: B,
    sender: Sender<()>,
    has_been_read: bool,
}

impl<B> fmt::Debug for ReadNotifier<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<B: Read> ReadNotifier<B> {
    pub(crate) fn new(reader: B, sender: Sender<()>) -> Self { panic!("STUB: not implemented") }
}

impl<B: BufRead> BufRead for ReadNotifier<B> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> { panic!("STUB: not implemented") }

    fn consume(self: Pin<&mut Self>, amt: usize) { panic!("STUB: not implemented") }
}

impl<B: Read> Read for ReadNotifier<B> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> { panic!("STUB: not implemented") }
}
