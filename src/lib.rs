// TODO(lucio): reenable this
// #![deny(missing_docs, warnings, missing_debug_implementations)]

//! Tower is a library of modular and reusable components for building robust networking
//! clients and servers.
//!
//! This main crate is still a WIP.

extern crate tower_middleware;
extern crate tower_service;
extern crate tower_util;

use std::marker::PhantomData;
use tower_middleware::{Chain, Middleware, MiddlewareExt};
use tower_service::Service;
use tower_util::MakeService;

pub struct ServiceBuilder<S: MakeService<Target, Request>, M, Target, Request> {
    maker: S,
    middleware: M,
    _pd: PhantomData<(Target, Request)>,
}

// impl<MakeService> ServiceBuilder<MakeService, Identity> {
//     pub fn new(maker: MakeService) -> Self {
//         ServiceBuilder {
//             maker,
//             middleware: Identity::new(),
//         }
//     }
// }

impl<S, M, Target, Request> ServiceBuilder<S, M, Target, Request>
where
    S: MakeService<Target, Request>,
    M: Middleware<S::Service, Request>,
{
    pub fn middleware<U: Middleware<S::Service, Request>>(
        self,
        middleware: U,
    ) -> ServiceBuilder<S, <M as Middleware<S::Service, Request>>::Response, Target, Request> {
        ServiceBuilder {
            maker: self.maker,
            middleware: self.middleware.chain(middleware),
            _pd: PhantomData,
        }
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
