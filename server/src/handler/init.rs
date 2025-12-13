use super::*;

pub async fn setup(
    addr: SocketAddr,
    ws_stream: WebSocketStream<TcpStream>,
    mut redis_conn: MultiplexedConnection,
) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("Addr:{addr} - Current State: Init");

    let uuid = loop {
        let uuid = Uuid::new_v4();
        let key = format!("user:{uuid}");
        let value = addr.to_string();

        let result: Result<bool, redis::RedisError> = redis_conn.hset_nx(&key, "ip", &value).await;
        let is_created = match result {
            Ok(is_created) => is_created,
            Err(e) => {
                eprintln!("Redis Error: {e}");
                return;
            }
        };

        if is_created {
            let result: Result<(), redis::RedisError> = redis::pipe()
                .hset(&key, WINS_KEY, 0)
                .hset(&key, LOSSES_KEY, 0)
                .hset(&key, DRAWS_KEY, 0)
                .query_async(&mut redis_conn)
                .await;

            if let Err(e) = result {
                eprintln!("Redis Error: {e}.");
                return;
            }

            break uuid;
        }
    };

    let player = Player::new(uuid, addr, ws_stream);
    let result = player.tx.send(Packet::Connection(PlayData {
        uuid: player.uuid(),
        name: player.name().to_string(),
        hero: player.hero(),
        win: player.win(),
        lose: player.lose(),
    }));
    if let Err(e) = result {
        println!("WebSocket disconnected ({:?}): {}", player, e);
        drop(player);
        return;
    }

    next_state(State::Title, Box::new(player), redis_conn);
}
