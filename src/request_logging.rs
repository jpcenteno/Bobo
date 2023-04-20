use log::debug;
use std::fmt;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone, Default)]
pub struct LogLayer;

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService { service }
    }
}

/// This service implements the Log behavior
#[derive(Clone, Default)]
pub struct LogService<S> {
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        debug!("Received request {:?}", request);
        self.service.call(request)
    }
}
