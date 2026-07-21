
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::len_zero)]
#![allow(clippy::match_bool)]
#![allow(clippy::unreadable_literal)]

const MAX_HEADERS: usize = 128;

const MAX_HEAD_LENGTH: usize = 8 * 1024;

mod body_encoder;
mod chunked;
mod date;
mod read_notifier;

pub mod client;
pub mod server;

use body_encoder::BodyEncoder;
pub use client::connect;
use futures_lite::io::Cursor;
pub use server::{accept, accept_with_opts, ServerOptions};

#[derive(Debug)]
pub(crate) enum EncoderState {
    Start,
    Head(Cursor<Vec<u8>>),
    Body(BodyEncoder),
    End,
}

#[macro_export]
macro_rules! read_to_end {
    ($expr:expr) => {
        match $expr {
            Poll::Ready(Ok(0)) => (),
            other => return other,
        }
    };
}
