extern crate futures;
extern crate tower;
extern crate tower_in_flight_limit;
extern crate tower_util;

use futures::future::{self, FutureResult};
use futures::Poll;
use tower::{MakeService, Service, ServiceBuilder};
use tower_in_flight_limit::InFlightLimitMiddleware;

fn main() {
    let builder = ServiceBuilder::new(Mock).middleware(InFlightLimitMiddleware::new(5));
}

struct Mock;

impl Service<()> for Mock {
    type Response = MockConn;
    type Error = ();
    type Future = FutureResult<MockConn, ()>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: ()) -> Self::Future {
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
