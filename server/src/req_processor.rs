use std::{future::Future, task::Poll};

use bytes::{BytesMut, BufMut};
use smdtp_lib::{error::SMDTPRequestError, request::SMDTPRequest, response::SMDTPResponse};
use tower::Service;

pub struct RequestProcessor;


impl Service<SMDTPRequest> for RequestProcessor {
    type Response = SMDTPResponse;

    type Error = SMDTPRequestError;

    type Future = RequestProcessorFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: SMDTPRequest) -> Self::Future {
        RequestProcessorFuture {
            req
        }
    }
}

pub struct RequestProcessorFuture {
    req: SMDTPRequest,
}

impl Future for RequestProcessorFuture {
    type Output = Result<SMDTPResponse, SMDTPRequestError>;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut bytes = BytesMut::with_capacity(16);
        for _ in 0..15 {
            bytes.put_u8(69);
        }
        println!("Processing request {:?}!", self.req);

        Poll::Ready(
            Ok(
                SMDTPResponse {
                    status: 0,
                    data: bytes
                }
            )
        )
    }
}