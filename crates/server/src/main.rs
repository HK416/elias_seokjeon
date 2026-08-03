mod handler;
mod stream;

use std::sync::OnceLock;

use tikv_jemallocator::Jemalloc;
use tokio::{
    net::TcpListener,
    time::{self, Duration},
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const EXPIRE_SECONDS: i64 = 15_552_000; // 180 days
const INITIAL_EXPIRE_SECONDS: i64 = 86_400; // 24 hours
const NAME_KEY: &str = "name";
const WINS_KEY: &str = "wins";
const LOSSES_KEY: &str = "losses";
const DRAWS_KEY: &str = "draws";
const LEADER_BOARD_KEY: &str = "leaderboard";

/// Set by nginx via `proxy_set_header X-Real-IP $remote_addr;`. Unlike
/// `X-Forwarded-For`, nginx *overwrites* this header rather than appending to
/// whatever the client sent, so its value can't be spoofed by the client.
const CLIENT_IP_HEADER: &str = "x-real-ip";

const NAMES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/name.txt"));
static NAME_TABLE: OnceLock<Vec<String>> = OnceLock::new();

pub fn get_name_table() -> &'static Vec<String> {
    NAME_TABLE.get_or_init(|| {
        let lines = NAMES.lines().map(|s| s.to_string());
        let set = Vec::from_iter(lines);
        set
    })
}

#[tokio::main]
async fn main() {
    // --- Init name table ---
    get_name_table();

    // --- Init Redis ---
    let result = redis::Client::open("redis://127.0.0.1/");
    let client = match result {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect to Redis: {e}");
            return;
        }
    };

    // ConnectionManager, not MultiplexedConnection: the latter never reconnects,
    // so a single Redis restart or network blip fails every later command for the
    // lifetime of the process.
    let result = client.get_connection_manager().await;
    let manager = match result {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Failed to connect to Redis: {e}");
            return;
        }
    };

    // --- Init matching queue ---
    let redis_conn = manager.clone();
    tokio::spawn(handler::matching::update(redis_conn));

    // --- Init WebSocket server ---
    let listener = TcpListener::bind("127.0.0.1:8889").await.unwrap();
    println!("WebSocket server listening on ws://127.0.0.1:8889");

    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(pair) => pair,
            Err(e) => {
                // Never break the loop: a transient accept error (EMFILE, a peer
                // resetting mid-handshake) would otherwise leave the process alive
                // but permanently deaf. Back off so a persistent one can't spin.
                eprintln!("Failed to accept TCP connection: {e}");
                time::sleep(Duration::from_millis(100)).await;
                continue;
            }
        };

        let redis_conn = manager.clone();
        // The WebSocket handshake must be awaited inside the spawned task. Awaiting
        // it here lets a single client that connects but never completes it stall
        // every other incoming connection.
        tokio::spawn(async move {
            // nginx terminates the client's TCP connection, so `peer` is always the
            // proxy's loopback address. The real client IP only reaches us as a
            // header on the WebSocket handshake request.
            let mut client_ip = None;
            let result = accept_hdr_async(stream, |req: &Request, res: Response| {
                client_ip = req
                    .headers()
                    .get(CLIENT_IP_HEADER)
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_owned);
                Ok(res)
            })
            .await;

            match result {
                Ok(ws_stream) => {
                    // Falls back to the peer address when the header is absent, so
                    // running without a proxy in front still works.
                    let addr = client_ip.unwrap_or_else(|| peer.to_string());
                    println!("New WebSocket connection (Address:{addr})");
                    handler::init::setup(addr, ws_stream, redis_conn).await;
                }
                Err(e) => {
                    eprintln!("Failed to accept WebSocket connection (Address:{peer}): {e}");
                }
            }
        });
    }
}
