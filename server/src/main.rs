use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8889").await.unwrap();
    println!("WebSocket server listening on ws://127.0.0.1:8889");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            println!("New WebSocket connection");

            let (mut write, mut read) = ws_stream.split();

            while let Some(msg) = read.next().await {
                let msg = msg.unwrap();
                println!("Received: {:?}", msg);

                if msg.is_text() || msg.is_binary() {
                    write.send(msg).await.unwrap();
                }
            }
        });
    }
}
