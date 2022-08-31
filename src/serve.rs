use futures::future::{self, BoxFuture, Ready};
use http::response::Builder as ResponseBuilder;
use http::{header, StatusCode};
use hyper::server::conn::AddrStream;
use hyper::service::Service;
use hyper::{Body, Request, Response};
use hyper_staticfile::Static;
use pin_project::pin_project;
use std::future::Future;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};

const PORT: u16 = 8000;

#[pin_project(project = MainFutureProj)]
enum MainFuture {
    Root,
    Static(#[pin] BoxFuture<'static, io::Result<Response<Body>>>),
}

impl Future for MainFuture {
    type Output = crate::Result<Response<Body>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.project() {
            MainFutureProj::Root => {
                let res = ResponseBuilder::new()
                    .status(StatusCode::MOVED_PERMANENTLY)
                    .header(header::LOCATION, "/rust-quiz/")
                    .body(Body::empty())
                    .map_err(crate::Error::Http);
                Poll::Ready(res)
            }
            MainFutureProj::Static(future) => future.poll(cx).map_err(crate::Error::Io),
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

impl Service<Request<Body>> for MainService {
    type Response = Response<Body>;
    type Error = crate::Error;
    type Future = MainFuture;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if req.uri().path() == "/" {
            MainFuture::Root
        } else {
            MainFuture::Static(Box::pin(self.staticfile.clone().serve(req)))
        }
    }
}

struct MakeMainService;

impl Service<&AddrStream> for MakeMainService {
    type Error = crate::Error;
    type Response = MainService;
    type Future = Ready<crate::Result<MainService>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _target: &AddrStream) -> Self::Future {
        future::ok(MainService::new())
    }
}

pub async fn main() -> crate::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT);
    let server = hyper::Server::try_bind(&addr)?.serve(MakeMainService);

    _ = writeln!(
        io::stderr(),
        "Quiz server running on http://localhost:{}/ ...",
        PORT,
    );

    server.await.map_err(crate::Error::Hyper)
}
