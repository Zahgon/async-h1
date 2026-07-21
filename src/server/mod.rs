
use async_io::Timer;
use futures_lite::io::{self, AsyncRead as Read, AsyncWrite as Write};
use futures_lite::prelude::*;
use http_types::headers::{CONNECTION, UPGRADE};
use http_types::upgrade::Connection;
use http_types::{Request, Response, StatusCode};
use std::{future::Future, marker::PhantomData, time::Duration};
mod body_reader;
mod decode;
mod encode;

pub use decode::decode;
pub use encode::Encoder;

#[derive(Debug, Clone)]
pub struct ServerOptions {
    
    headers_timeout: Option<Duration>,
}

impl Default for ServerOptions {
    fn default() -> Self { panic!("STUB: not implemented") }
}

pub async fn accept<RW, F, Fut>(io: RW, endpoint: F) -> http_types::Result<()>
where
    RW: Read + Write + Clone + Send + Sync + Unpin + 'static,
    F: Fn(Request) -> Fut,
    Fut: Future<Output = http_types::Result<Response>>,
{ panic!("STUB: not implemented") }

pub async fn accept_with_opts<RW, F, Fut>(
    io: RW,
    endpoint: F,
    opts: ServerOptions,
) -> http_types::Result<()>
where
    RW: Read + Write + Clone + Send + Sync + Unpin + 'static,
    F: Fn(Request) -> Fut,
    Fut: Future<Output = http_types::Result<Response>>,
{ panic!("STUB: not implemented") }

#[derive(Debug)]
pub struct Server<RW, F, Fut> {
    io: RW,
    endpoint: F,
    opts: ServerOptions,
    _phantom: PhantomData<Fut>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConnectionStatus {
    
    Close,

    KeepAlive,
}

impl<RW, F, Fut> Server<RW, F, Fut>
where
    RW: Read + Write + Clone + Send + Sync + Unpin + 'static,
    F: Fn(Request) -> Fut,
    Fut: Future<Output = http_types::Result<Response>>,
{
    
    pub fn new(io: RW, endpoint: F) -> Self { panic!("STUB: not implemented") }

    pub fn with_opts(mut self, opts: ServerOptions) -> Self { panic!("STUB: not implemented") }

    pub async fn accept(&mut self) -> http_types::Result<()> { panic!("STUB: not implemented") }

    pub async fn accept_one(&mut self) -> http_types::Result<ConnectionStatus>
    where
        RW: Read + Write + Clone + Send + Sync + Unpin + 'static,
        F: Fn(Request) -> Fut,
        Fut: Future<Output = http_types::Result<Response>>,
    { panic!("STUB: not implemented") }
}
