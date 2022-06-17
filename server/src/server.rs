use bytes::BytesMut;
use smdtp_lib::{
    error::SMDTPRequestError, middleware::auth::RequestAuthentication, request::SMDTPRequest, response::SMDTPResponse,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tower::Service;

use crate::req_processor::RequestProcessor;

pub struct Server {
    port: String,
}

impl Server {
    pub fn new(port: &str) -> Self {
        Self {
            port: port.to_owned(),
        }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(&self.port).await.unwrap();

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = BytesMut::with_capacity(16);

                if let Err(_) = socket.read_buf(&mut buf).await {
                    return Err(SMDTPRequestError::InternalServerError);
                }

                let request = match SMDTPRequest::try_from(buf) {
                    Ok(request) => request,
                    Err(error) => {
                        return Err(error);
                    }
                };

                let handler = RequestProcessor;

                let auth: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
                let handler = RequestAuthentication::new(handler, auth).call(request).await;

                match handler {
                    Ok(res) => {

                        println!("Sending success response...");
                        socket.write(&res.as_bytes()).await.unwrap();
                        println!("Response sent!");
                    },
                    Err(error) => {
                        let response = SMDTPResponse::from(error);
                        println!("Sending error response...");
                        println!("Response btw: {response:?}");
                        socket.write(&response.as_bytes()).await.unwrap();
                        println!("Response sent!");
                    }
                }
                Ok(())
            });
        }
    }
}
