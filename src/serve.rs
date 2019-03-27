use futures::{future, Async, Future, Poll};
use http::response::Builder as ResponseBuilder;
use http::{header, Request, Response, StatusCode};
use hyper::Body;
use hyper_staticfile::{Static, StaticFuture};

use std::io::{self, Error, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

use crate::error::Result;

const PORT: u16 = 8000;

enum MainFuture {
    Root,
    Static(StaticFuture<Body>),
}

impl Future for MainFuture {
    type Item = Response<Body>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self {
            MainFuture::Root => {
                let res = ResponseBuilder::new()
                    .status(StatusCode::MOVED_PERMANENTLY)
                    .header(header::LOCATION, "/rust-quiz/")
                    .body(Body::empty())
                    .expect("unable to build response");
                Ok(Async::Ready(res))
            }
            MainFuture::Static(future) => future.poll(),
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

impl hyper::service::Service for MainService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = MainFuture;

    fn call(&mut self, req: Request<Body>) -> MainFuture {
        if req.uri().path() == "/" {
            MainFuture::Root
        } else {
            MainFuture::Static(self.staticfile.serve(req))
        }
    }
}

pub fn main() -> Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT);

    let server = hyper::Server::try_bind(&addr)?
        .serve(|| future::ok::<_, Error>(MainService::new()))
        .map_err(|e| {
            let _ = writeln!(io::stderr(), "server error: {}", e);
        });

    let _ = writeln!(
        io::stderr(),
        "Quiz server running on http://localhost:{}/ ...",
        PORT,
    );

    hyper::rt::run(server);

    Ok(())
}
