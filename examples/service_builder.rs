// extern crate futures;
// extern crate http;
// extern crate hyper;
// extern crate tower;
// extern crate tower_hyper;
// extern crate tower_in_flight_limit;
// extern crate tower_rate_limit;
// extern crate tower_util;

// use futures::Future;
// use http::Uri;
// use hyper::client::connect::Destination;
// use hyper::client::HttpConnector;
// use hyper::{Body, Request, Response};
// use std::time::Duration;
// use tower::ServiceBuilder;
// use tower_hyper::client::{Builder, Connect};
// use tower_hyper::util::Connector;
// use tower_in_flight_limit::InFlightLimitMiddleware;
// use tower_rate_limit::RateLimitMiddleware;

// fn main() {
//     hyper::rt::run(request.map(|_| ()).map_err(|_| ()))
// }

// fn request() -> impl Future<Item = Response<Body>, Error = hyper::Error> {
//     let dst = Destination::try_from_uri(Uri::from_static("http://127.0.0.1:3000")).unwrap();
//     let connector = Connector::new(HttpConnector::new(1));
//     let hyper = Connect::new(connector, Builder::new());

//     let client = ServiceBuilder::new(hyper)
//         .middleware(InFlightLimitMiddleware::new(5))
//         .middleware(RateLimitMiddleware::new(5, Duration::from_secs(1)))
//         .build(dst);

//     let request = Request::builder()
//         .method("GET")
//         .body(Body::empty())
//         .unwrap();

//     client.call(request)
// }
fn main() {}
