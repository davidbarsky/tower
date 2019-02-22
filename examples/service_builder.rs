extern crate futures;
extern crate tower;
extern crate tower_in_flight_limit;
extern crate tower_rate_limit;
extern crate tower_util;

use futures::future::{self, FutureResult};
use futures::Poll;
use std::time::Duration;
use tower::{MakeService, Service, ServiceBuilder};
use tower_in_flight_limit::InFlightLimitMiddleware;
use tower_rate_limit::RateLimitMiddleware;

fn main() {
    let builder = ServiceBuilder::new(Mock)
        .middleware(InFlightLimitMiddleware::new(5))
        .middleware(RateLimitMiddleware::new(5, Duration::from_secs(1)))
        .build("https://google.com".to_owned());
}

struct Mock;

impl Service<String> for Mock {
    type Response = MockConn;
    type Error = ();
    type Future = FutureResult<MockConn, ()>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: String) -> Self::Future {
        future::ok(MockConn)
    }
}

struct MockConn;

impl Service<()> for MockConn {
    type Response = ();
    type Error = ();
    type Future = FutureResult<(), ()>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: ()) -> Self::Future {
        future::ok(())
    }
}
