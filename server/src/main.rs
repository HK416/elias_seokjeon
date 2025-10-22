mod handler;
mod stream;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8889").await.unwrap();
    println!("WebSocket server listening on ws://0.0.0.0:8889");

    tokio::spawn(handler::matching::update());

    while let Ok((stream, addr)) = listener.accept().await {
        let result = accept_async(stream).await;
        let ws_stream = match result {
            Ok(ws_stream) => {
                println!("New WebSocket connection (Address:{addr})");
                ws_stream
            }
            Err(e) => {
                eprintln!("Failed to accept WebSocket connection (Address:{addr}): {e}");
                return;
            }
        };

        tokio::spawn(handler::init::setup(addr, ws_stream));
    }
}
