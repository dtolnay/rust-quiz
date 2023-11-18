use crate::Error;
use futures::future::BoxFuture;
use http::response::Builder as ResponseBuilder;
use http::{header, Request, Response, StatusCode};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper_staticfile::{Body, Static};
use hyper_util::rt::TokioIo;
use pin_project::pin_project;
use std::future::Future;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpListener;

const PORT: u16 = 8000;

#[pin_project(project = MainFutureProj)]
enum MainFuture {
    Root,
    Static(#[pin] BoxFuture<'static, io::Result<Response<Body>>>),
}

impl Future for MainFuture {
    type Output = Result<Response<Body>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.project() {
            MainFutureProj::Root => {
                let res = ResponseBuilder::new()
                    .status(StatusCode::MOVED_PERMANENTLY)
                    .header(header::LOCATION, "/rust-quiz/")
                    .body(Body::Empty)
                    .map_err(Error::Http);
                Poll::Ready(res)
            }
            MainFutureProj::Static(future) => future.poll(cx).map_err(Error::Io),
        }
    }
}

struct MainService {
    staticfile: Static,
}

impl MainService {
    fn new() -> MainService {
        MainService {
            staticfile: Static::new(Path::new(".")),
        }
    }
}

impl Service<Request<Incoming>> for MainService {
    type Response = Response<Body>;
    type Error = Error;
    type Future = MainFuture;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        if req.uri().path() == "/" {
            MainFuture::Root
        } else {
            MainFuture::Static(Box::pin(self.staticfile.clone().serve(req)))
        }
    }
}

pub async fn main() -> Result<(), Error> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT);
    let listener = TcpListener::bind(addr).await?;

    let _ = writeln!(
        io::stderr(),
        "Quiz server running on http://localhost:{}/ ...",
        PORT,
    );

    loop {
        let (tcp_stream, _socket_addr) = listener.accept().await?;
        let io = TokioIo::new(tcp_stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, MainService::new())
                .await
            {
                let _ = writeln!(io::stderr(), "{}", err);
            }
        });
    }
}
