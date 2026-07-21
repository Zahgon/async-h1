use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::io::AsyncRead as Read;
use futures_lite::{io, ready};

#[derive(Debug)]
pub(crate) struct ChunkedEncoder<R> {
    reader: R,
    done: bool,
}

impl<R: Read + Unpin> ChunkedEncoder<R> {
    
    pub(crate) fn new(reader: R) -> Self { panic!("STUB: not implemented") }
}

impl<R: Read + Unpin> Read for ChunkedEncoder<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> { panic!("STUB: not implemented") }
}

fn max_bytes_to_read(buf_len: usize) -> usize { panic!("STUB: not implemented") }

#[cfg(test)]
mod test_bytes_to_read {
    #[test]
    fn simple_check_of_known_values() {
        
        let values = vec![
            (6, 1),       
            (7, 2),       
            (20, 15),     
            (21, 15),     
            (22, 16),     
            (23, 17),     
            (260, 254),   
            (261, 254),   
            (262, 255),   
            (263, 256),   
            (4100, 4093), 
            (4101, 4093), 
            (4102, 4094), 
            (4103, 4095), 
            (4104, 4096), 
        ];

        for (input, expected) in values {
            let actual = super::max_bytes_to_read(input);
            assert_eq!(
                actual, expected,
                "\n\nexpected max_bytes_to_read({}) to be {}, but it was {}",
                input, expected, actual
            );

            let used_bytes = expected + 4 + format!("{:X}", expected).len();
            assert!(
                used_bytes == input || used_bytes == input - 1,
                "\n\nfor an input of {}, expected used bytes to be {} or {}, but was {}",
                input,
                input,
                input - 1,
                used_bytes
            );
        }
    }
}
