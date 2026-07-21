
use std::str::FromStr;

use async_dup::{Arc, Mutex};
use futures_lite::io::{AsyncRead as Read, AsyncWrite as Write, BufReader};
use futures_lite::prelude::*;
use http_types::content::ContentLength;
use http_types::headers::{EXPECT, TRANSFER_ENCODING};
use http_types::{ensure, ensure_eq, format_err};
use http_types::{Body, Method, Request, Url};

use super::body_reader::BodyReader;
use crate::chunked::ChunkedDecoder;
use crate::read_notifier::ReadNotifier;
use crate::{MAX_HEADERS, MAX_HEAD_LENGTH};

const LF: u8 = b'\n';

const HTTP_1_1_VERSION: u8 = 1;

const CONTINUE_HEADER_VALUE: &str = "100-continue";
const CONTINUE_RESPONSE: &[u8] = b"HTTP/1.1 100 Continue\r\n\r\n";

pub async fn decode<IO>(mut io: IO) -> http_types::Result<Option<(Request, BodyReader<IO>)>>
where
    IO: Read + Write + Clone + Send + Sync + Unpin + 'static,
{ panic!("STUB: not implemented") }

fn url_from_httparse_req(req: &httparse::Request<'_, '_>) -> http_types::Result<Url> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    fn httparse_req(buf: &str, f: impl Fn(httparse::Request<'_, '_>)) {
        let mut headers = [httparse::EMPTY_HEADER; MAX_HEADERS];
        let mut res = httparse::Request::new(&mut headers[..]);
        res.parse(buf.as_bytes()).unwrap();
        f(res)
    }

    #[test]
    fn url_for_connect() {
        httparse_req(
            "CONNECT server.example.com:443 HTTP/1.1\r\nHost: server.example.com:443\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(url.as_str(), "http://server.example.com:443/");
            },
        );
    }

    #[test]
    fn url_for_host_plus_path() {
        httparse_req(
            "GET /some/resource HTTP/1.1\r\nHost: server.example.com:443\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(url.as_str(), "http://server.example.com:443/some/resource");
            },
        )
    }

    #[test]
    fn url_for_host_plus_absolute_url() {
        httparse_req(
            "GET http://domain.com/some/resource HTTP/1.1\r\nHost: server.example.com\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(url.as_str(), "http://domain.com/some/resource"); 
            },
        )
    }

    #[test]
    fn url_for_conflicting_connect() {
        httparse_req(
            "CONNECT server.example.com:443 HTTP/1.1\r\nHost: conflicting.host\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(url.as_str(), "http://server.example.com:443/");
            },
        )
    }

    #[test]
    fn url_for_malformed_resource_path() {
        httparse_req(
            "GET not-a-url HTTP/1.1\r\nHost: server.example.com\r\n",
            |req| {
                assert!(url_from_httparse_req(&req).is_err());
            },
        )
    }

    #[test]
    fn url_for_double_slash_path() {
        httparse_req(
            "GET //double/slashes HTTP/1.1\r\nHost: server.example.com:443\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(
                    url.as_str(),
                    "http://server.example.com:443//double/slashes"
                );
            },
        )
    }
    #[test]
    fn url_for_triple_slash_path() {
        httparse_req(
            "GET ///triple/slashes HTTP/1.1\r\nHost: server.example.com:443\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(
                    url.as_str(),
                    "http://server.example.com:443///triple/slashes"
                );
            },
        )
    }

    #[test]
    fn url_for_query() {
        httparse_req(
            "GET /foo?bar=1 HTTP/1.1\r\nHost: server.example.com:443\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(url.as_str(), "http://server.example.com:443/foo?bar=1");
            },
        )
    }

    #[test]
    fn url_for_anchor() {
        httparse_req(
            "GET /foo?bar=1#anchor HTTP/1.1\r\nHost: server.example.com:443\r\n",
            |req| {
                let url = url_from_httparse_req(&req).unwrap();
                assert_eq!(
                    url.as_str(),
                    "http://server.example.com:443/foo?bar=1#anchor"
                );
            },
        )
    }
}
