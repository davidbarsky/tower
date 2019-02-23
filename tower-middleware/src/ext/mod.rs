use tower_service::Service;
use Middleware;

mod chain;

pub use self::chain::Chain;

/// An extension trait for `Middleware`'s that provides a variety of convenient
/// adapters.
pub trait MiddlewareExt<S, Request>: Middleware<S, Request> {
    /// Return a new `Middleware` instance that applies both `self` and
    /// `middleware` to services being wrapped.
    ///
    /// This defines a middleware stack.
    fn chain<T, SS>(self, middleware: T) -> Chain<Self, T>
    where
        T: Middleware<SS, Request>,
        SS: Service<Request>,
        Self: Sized,
    {
        Chain::new(self, middleware)
    }
}

impl<T, S, Request> MiddlewareExt<S, Request> for T where T: Middleware<S, Request> {}
