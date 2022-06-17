use std::{future::Future, task::Poll};

use bytes::BytesMut;
use pin_project::pin_project;
use tower::Service;

use crate::{request::SMDTPRequest, response::SMDTPResponse, error::SMDTPRequestError};

/// A struct for authenticating requests, it takes an auth code to match against.
pub struct RequestAuthentication<S> {
    inner: S,
    auth_code: [u8; 8],
}

impl<S> RequestAuthentication<S> {
    pub fn new(inner: S, auth_code: [u8; 8]) -> Self {
        Self { inner, auth_code }
    }
}

/// The Service implementation for RequestAuthentication
impl<S> Service<SMDTPRequest> for RequestAuthentication<S>
where
    // The two bounds below are to make sure that the inner Service has the exact same signature as the auth service.
    S: Service<SMDTPRequest>,
    S::Future: Future<Output = Result<SMDTPResponse, SMDTPRequestError>>,
{
    type Response = SMDTPResponse;

    type Error = SMDTPRequestError;

    type Future = RequestAuthenticationFuture<S::Future>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    /// Call function provided by the Service trait
    fn call(&mut self, req: SMDTPRequest) -> Self::Future {
        let bytes = req.data().clone();
        let response_future = self.inner.call(req);

        RequestAuthenticationFuture {
            inner: response_future,
            auth_code: self.auth_code,
            request: bytes,
        }
    }
}

#[pin_project]
pub struct RequestAuthenticationFuture<F> {
    #[pin]
    inner: F,
    request: BytesMut,
    auth_code: [u8; 8],
}

impl<F> Future for RequestAuthenticationFuture<F>
where
    F: Future<Output = Result<SMDTPResponse, SMDTPRequestError>>,
{
    type Output = Result<SMDTPResponse, SMDTPRequestError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        // Get the bytes from the request
        let array_bytes = &this.request[0..7];

        // Loop through the array and request-bytes-array
        for i in 0..7 {
            if this.auth_code[i] != array_bytes[i] {
                // If one byte doesn't match then throw an error.
                return Poll::Ready(Err(SMDTPRequestError::Forbidden));
            }
        }

        match this.inner.poll(cx) {
            Poll::Ready(data) => Poll::Ready(data),
            Poll::Pending => Poll::Pending,
        }
    }
}
