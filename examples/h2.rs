extern crate futures;
extern crate http;
extern crate tokio;
// extern crate tower;
extern crate tower_h2;
extern crate tower_in_flight_limit;
extern crate tower_rate_limit;
extern crate tower_service;
extern crate tower_util;

use futures::{Future, Poll};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::executor::DefaultExecutor;
use tokio::net::TcpStream;
// use tower::ServiceBuilder;
use tower_h2::client::Connect;
use tower_in_flight_limit::InFlightLimitMiddleware;
use tower_rate_limit::RateLimitMiddleware;
use tower_service::Service;

fn main() {
    // tokio::run(request.map(|_| ()).map_err(|_| ()))
    request();
}

fn request() {
    //-> impl Future<Item = Response<Body>, Error = hyper::Error> {
    let conn = Conn("127.0.0.1:8888".parse().unwrap());

    let connect = Connect::new(conn, Default::default(), DefaultExecutor::current());

    // let client = ServiceBuilder::new(connect)
    //     .middleware(InFlightLimitMiddleware::new(5))
    //     .middleware(RateLimitMiddleware::new(5, Duration::from_secs(1)))
    //     .build(());

    // let request = Request::builder()
    //     .method("GET")
    //     .body(Body::empty())
    //     .unwrap();

    // client.call(request)
}

pub struct Conn(SocketAddr);

impl Service<()> for Conn {
    type Response = TcpStream;
    type Error = ::std::io::Error;
    type Future = Box<Future<Item = TcpStream, Error = ::std::io::Error> + Send>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: ()) -> Self::Future {
        let c = TcpStream::connect(&self.0).and_then(|tcp| tcp.set_nodelay(true).map(move |_| tcp));
        Box::new(c)
    }
}
