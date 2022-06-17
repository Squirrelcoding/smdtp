use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data.
    stream.write_all(&[69, 0, 1, 2, 3, 4, 5, 6, 7]).await?;

    let mut buf = BytesMut::with_capacity(16);
    buf.resize(16, 0);
    match stream.read(&mut buf).await {
        Ok(n) => {
            println!("{n}");
            println!("{:?}", buf);
        },
        Err(err) => {
            println!("{err}");
        }
    }

    Ok(())
}