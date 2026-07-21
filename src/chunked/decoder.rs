use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::io::{self, AsyncRead as Read};
use futures_lite::ready;
use http_types::trailers::{Sender, Trailers};

#[derive(Debug)]
pub struct ChunkedDecoder<R: Read> {
    
    inner: R,
    
    state: State,
    
    chunk_size: u64,
    
    trailer_sender: Option<Sender>,
}

impl<R: Read> ChunkedDecoder<R> {
    pub(crate) fn new(inner: R, trailer_sender: Sender) -> Self { panic!("STUB: not implemented") }
}

enum State {
    
    ChunkSize,
    
    ChunkSizeExpectLf,
    
    ChunkBody,
    
    ChunkBodyExpectCr,
    
    ChunkBodyExpectLf,
    
    Trailers(usize, Box<[u8; 8192]>),
    
    TrailerSending(Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>),
    
    Done,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<R: Read + Unpin> ChunkedDecoder<R> {
    fn poll_read_byte(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<u8>> { panic!("STUB: not implemented") }

    fn expect_byte(
        &mut self,
        cx: &mut Context<'_>,
        expected_byte: u8,
        expected: &'static str,
    ) -> Poll<io::Result<()>> { panic!("STUB: not implemented") }

    fn send_trailers(&mut self, trailers: Trailers) { panic!("STUB: not implemented") }
}

fn eof<T>() -> Poll<io::Result<T>> { panic!("STUB: not implemented") }

fn unexpected<T>(byte: u8, expected: &'static str) -> Poll<io::Result<T>> { panic!("STUB: not implemented") }

fn overflow() -> io::Error { panic!("STUB: not implemented") }

impl<R: Read + Unpin> Read for ChunkedDecoder<R> {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::prelude::*;

    #[test]
    fn test_chunked_wiki() {
        async_std::task::block_on(async move {
            let input = async_std::io::Cursor::new(
                "4\r\n\
                  Wiki\r\n\
                  5\r\n\
                  pedia\r\n\
                  E\r\n in\r\n\
                  \r\n\
                  chunks.\r\n\
                  0\r\n\
                  \r\n"
                    .as_bytes(),
            );

            let (s, _r) = async_channel::bounded(1);
            let sender = Sender::new(s);
            let mut decoder = ChunkedDecoder::new(input, sender);

            let mut output = String::new();
            decoder.read_to_string(&mut output).await.unwrap();
            assert_eq!(
                output,
                "Wikipedia in\r\n\
                 \r\n\
                 chunks."
            );
        });
    }

    #[test]
    fn test_chunked_big() {
        async_std::task::block_on(async move {
            let mut input: Vec<u8> = b"800\r\n".to_vec();
            input.extend(vec![b'X'; 2048]);
            input.extend(b"\r\n1800\r\n");
            input.extend(vec![b'Y'; 6144]);
            input.extend(b"\r\n800\r\n");
            input.extend(vec![b'Z'; 2048]);
            input.extend(b"\r\n0\r\n\r\n");

            let (s, _r) = async_channel::bounded(1);
            let sender = Sender::new(s);
            let mut decoder = ChunkedDecoder::new(async_std::io::Cursor::new(input), sender);

            let mut output = String::new();
            decoder.read_to_string(&mut output).await.unwrap();

            let mut expected = vec![b'X'; 2048];
            expected.extend(vec![b'Y'; 6144]);
            expected.extend(vec![b'Z'; 2048]);
            assert_eq!(output.len(), 10240);
            assert_eq!(output.as_bytes(), expected.as_slice());
        });
    }

    #[test]
    fn test_chunked_mdn() {
        async_std::task::block_on(async move {
            let input = async_std::io::Cursor::new(
                "7\r\n\
                 Mozilla\r\n\
                 9\r\n\
                 Developer\r\n\
                 7\r\n\
                 Network\r\n\
                 0\r\n\
                 Expires: Wed, 21 Oct 2015 07:28:00 GMT\r\n\
                 \r\n"
                    .as_bytes(),
            );
            let (s, r) = async_channel::bounded(1);
            let sender = Sender::new(s);
            let mut decoder = ChunkedDecoder::new(input, sender);

            let mut output = String::new();
            decoder.read_to_string(&mut output).await.unwrap();
            assert_eq!(output, "MozillaDeveloperNetwork");

            let trailers = r.recv().await.unwrap();
            assert_eq!(trailers.iter().count(), 1);
            assert_eq!(trailers["Expires"], "Wed, 21 Oct 2015 07:28:00 GMT");
        });
    }

    #[test]
    fn test_ff7() {
        async_std::task::block_on(async move {
            let mut input: Vec<u8> = b"FF7\r\n".to_vec();
            input.extend(vec![b'X'; 0xFF7]);
            input.extend(b"\r\n4\r\n");
            input.extend(vec![b'Y'; 4]);
            input.extend(b"\r\n0\r\n\r\n");

            let (s, _r) = async_channel::bounded(1);
            let sender = Sender::new(s);
            let mut decoder = ChunkedDecoder::new(async_std::io::Cursor::new(input), sender);

            let mut output = String::new();
            decoder.read_to_string(&mut output).await.unwrap();
            assert_eq!(
                output,
                "X".to_string().repeat(0xFF7) + &"Y".to_string().repeat(4)
            );
        });
    }
}
