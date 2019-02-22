// TODO(lucio): reenable this
// #![deny(missing_docs, warnings, missing_debug_implementations)]

//! Tower is a library of modular and reusable components for building robust networking
//! clients and servers.
//!
//! This main crate is still a WIP.

extern crate tower_middleware;
extern crate tower_reconnect;
extern crate tower_service;
extern crate tower_util;

use std::marker::PhantomData;
use tower_middleware::{Chain, Middleware, MiddlewareExt};
use tower_reconnect::Reconnect;
pub use tower_service::Service;
pub use tower_util::MakeService;

pub struct ServiceBuilder<S, M, Target, Request>
where
    S: Service<Target>,
    S::Response: Service<Request>,
{
    maker: S,
    middleware: M,
    _pd: PhantomData<(Target, Request)>,
}

impl<M, Target, Request> ServiceBuilder<M, Identity, Target, Request>
where
    M: Service<Target>,
    M::Response: Service<Request>,
{
    pub fn new(maker: M) -> Self {
        ServiceBuilder {
            maker,
            middleware: Identity::new(),
            _pd: PhantomData,
        }
    }
}

impl<S, M, Target, Request> ServiceBuilder<S, M, Target, Request>
where
    S: Service<Target>,
    S::Response: Service<Request>,
    M: Middleware<S::Response, Request>,
{
    pub fn middleware<U: Middleware<M::Service, Request>>(
        self,
        middleware: U,
    ) -> ServiceBuilder<S, Chain<M, U>, Target, Request> {
        ServiceBuilder {
            maker: self.maker,
            middleware: self.middleware.chain(middleware),
            _pd: PhantomData,
        }
    }
}

impl<S, M, Target, Request> ServiceBuilder<S, M, Target, Request>
where
    S: Service<Target>,
    S::Response: Service<Request>,
    M: Middleware<Reconnect<S, Target>, Request>,
    Target: Clone,
{
    pub fn build(self, target: Target) -> M::Service {
        let reconnect = Reconnect::new(self.maker, target);
        self.middleware.wrap(reconnect)
    }
}

/// A no-op middleware.
///
/// When wrapping a `Service`, the `Identity` middleware returns the provided
/// service without modifying it.
#[derive(Debug, Default, Clone)]
pub struct Identity {
    _p: (),
}

impl Identity {
    /// Create a new `Identity` value
    pub fn new() -> Identity {
        Identity { _p: () }
    }
}

/// Decorates a `Service`, transforming either the request or the response.
impl<S: Service<Request>, Request> Middleware<S, Request> for Identity {
    type Response = S::Response;
    type Error = S::Error;
    type Service = S;

    fn wrap(&self, inner: S) -> Self::Service {
        inner
    }
}
