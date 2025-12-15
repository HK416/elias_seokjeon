mod handler;
mod stream;

use std::sync::OnceLock;

use tikv_jemallocator::Jemalloc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const EXPIRE_SECONDS: i64 = 15_552_000; // 180 days
const INITIAL_EXPIRE_SECONDS: i64 = 86_400; // 24 hours
const NAME_KEY: &str = "name";
const WINS_KEY: &str = "wins";
const LOSSES_KEY: &str = "losses";
const DRAWS_KEY: &str = "draws";
const LEADER_BOARD_KEY: &str = "leaderboard";

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

    let result = client.get_multiplexed_async_connection().await;
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
    let listener = TcpListener::bind("127.0.0.0:8889").await.unwrap();
    println!("WebSocket server listening on ws://127.0.0.0:8889");

    while let Ok((stream, addr)) = listener.accept().await {
        let result = accept_async(stream).await;
        let ws_stream = match result {
            Ok(ws_stream) => {
                println!("New WebSocket connection (Address:{addr})");
                ws_stream
            }
            Err(e) => {
                eprintln!("Failed to accept WebSocket connection (Address:{addr}): {e}");
                continue;
            }
        };

        let redis_conn = manager.clone();
        tokio::spawn(handler::init::setup(addr, ws_stream, redis_conn));
    }
}
