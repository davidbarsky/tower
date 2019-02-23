#![deny(missing_docs, warnings, missing_debug_implementations)]

//! Tower is a library of modular and reusable components for building robust networking
//! clients and servers.
//!
//! This main crate is still a WIP.

extern crate futures;
extern crate tower_middleware;
extern crate tower_service;

use futures::{Future, Poll};
use std::marker::PhantomData;
use std::sync::Arc;
use tower_middleware::{Chain, Middleware, MiddlewareExt};
use tower_service::Service;

/// Configure and build a `MakeService`
///
/// `ServiceBuilder` collects middleware and a `MakeService` transport
/// and produces a new `MakeService` that is wrapped by the composed
/// middleware.
#[derive(Debug)]
pub struct ServiceBuilder<S, M, Target, Request>
where
    S: Service<Target>,
    S::Response: Service<Request>,
{
    maker: S,
    middleware: M,
    _pd: PhantomData<(Target, Request)>,
}

/// Composed `MakeService` produced from `ServiceBuilder`
#[derive(Debug)]
pub struct ServiceBuilderMaker<S, M, Target, Request> {
    maker: S,
    middleware: Arc<M>,
    _pd: PhantomData<(Target, Request)>,
}

impl<M, Target, Request> ServiceBuilder<M, Identity, Target, Request>
where
    M: Service<Target>,
    M::Response: Service<Request>,
{
    /// Create a new `ServiceBuilder` from a `MakeService`.
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
    Target: Clone,
{
    /// Add a middleware to the `ServiceBuilder`.
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

    /// Create a `ServiceBuilderMaker` from the composed middleware and transport.
    pub fn build(self) -> ServiceBuilderMaker<S, M, Target, Request> {
        ServiceBuilderMaker {
            maker: self.maker,
            middleware: Arc::new(self.middleware),
            _pd: PhantomData,
        }
    }
}

impl<S, M, Target, Request> Service<Target> for ServiceBuilderMaker<S, M, Target, Request>
where
    S: Service<Target> + 'static,
    S::Response: Service<Request> + 'static,
    S::Error: 'static,
    S::Future: 'static,
    M: Middleware<S::Response, Request> + 'static,
    M::Service: 'static,
    Target: Clone,
{
    type Response = M::Service;
    type Error = S::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.maker.poll_ready()
    }

    fn call(&mut self, target: Target) -> Self::Future {
        let middleware = Arc::clone(&self.middleware);

        let fut = self
            .maker
            .call(target)
            .and_then(move |conn| Ok(middleware.wrap(conn)));

        // TODO(lucio): replace this with a concrete future type
        Box::new(fut)
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
