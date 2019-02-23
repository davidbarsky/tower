extern crate futures;
extern crate http;
extern crate hyper;
extern crate tower;
extern crate tower_buffer;
extern crate tower_hyper;
extern crate tower_in_flight_limit;
extern crate tower_middleware;
extern crate tower_rate_limit;
extern crate tower_reconnect;
extern crate tower_retry;
extern crate tower_service;
extern crate tower_util;

use futures::Future;
use http::Uri;
use hyper::client::connect::Destination;
use hyper::client::HttpConnector;
use hyper::{Request, Response};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_buffer::BufferMiddleware;
use tower_hyper::client::{Builder, Connect};
use tower_hyper::retries::RetryPolicy;
use tower_hyper::util::Connector;
use tower_hyper::Body;
use tower_in_flight_limit::InFlightLimitMiddleware;
use tower_rate_limit::RateLimitMiddleware;
use tower_reconnect::Reconnect;
use tower_retry::RetryMiddleware;
use tower_service::Service;

fn main() {
    hyper::rt::run(futures::lazy(|| request().map(|_| ())))
}

fn request() -> impl Future<Item = Response<hyper::Body>, Error = ()> {
    let connector = Connector::new(HttpConnector::new(1));
    let hyper = Connect::new(connector, Builder::new());

    let policy = RetryPolicy::new(5);
    let dst = Destination::try_from_uri(Uri::from_static("http://127.0.0.1:3000")).unwrap();

    let maker = ServiceBuilder::new(hyper)
        .middleware(BufferMiddleware::new(5))
        .middleware(RetryMiddleware::new(policy))
        .middleware(InFlightLimitMiddleware::new(5))
        .middleware(RateLimitMiddleware::new(5, Duration::from_secs(1)))
        .build();

    let mut client = Reconnect::new(maker, dst);

    let request = Request::builder()
        .method("GET")
        .body(Body::from(Vec::new()))
        .unwrap();

    client.call(request).map_err(|e| panic!("{:?}", e))
}
