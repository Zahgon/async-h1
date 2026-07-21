use futures_lite::io::{AsyncRead as Read, BufReader};
use futures_lite::prelude::*;
use http_types::{ensure, ensure_eq, format_err};
use http_types::{
    headers::{CONTENT_LENGTH, DATE, TRANSFER_ENCODING},
    Body, Response, StatusCode,
};

use std::convert::TryFrom;

use crate::chunked::ChunkedDecoder;
use crate::date::fmt_http_date;
use crate::{MAX_HEADERS, MAX_HEAD_LENGTH};

const CR: u8 = b'\r';
const LF: u8 = b'\n';

pub async fn decode<R>(reader: R) -> http_types::Result<Response>
where
    R: Read + Unpin + Send + Sync + 'static,
{ panic!("STUB: not implemented") }
