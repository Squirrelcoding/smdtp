use server::Server;

mod server;
mod req_processor;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run().await;
}