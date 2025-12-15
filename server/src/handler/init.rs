use super::*;

const MAX_UUID_RETRIES: u32 = 10;

pub async fn setup(
    addr: SocketAddr,
    ws_stream: WebSocketStream<TcpStream>,
    mut redis_conn: MultiplexedConnection,
) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("Addr:{addr} - Current State: Init");

    let hero = rand::random();
    let prefix = get_name_table().choose(&mut rand::rng()).unwrap();
    let name = format!("{prefix} {hero}");

    let mut final_uuid = None;
    // --- Try to create a new user with a unique UUID, with a limited number of retries ---
    for _ in 0..MAX_UUID_RETRIES {
        let uuid = Uuid::new_v4();
        let key = format!("user:{uuid}");
        let value = addr.to_string();

        let is_created: bool = match redis_conn.hset_nx(&key, "ip", &value).await {
            Ok(created) => created,
            Err(e) => {
                eprintln!("Redis Error on HSETNX: {e}");
                // In case of error, stop trying.
                return;
            }
        };

        if is_created {
            // If the UUID is unique and 'ip' field is set, populate the rest of the user data.
            let pipe_result: Result<(), redis::RedisError> = redis::pipe()
                .hset(&key, NAME_KEY, &name)
                .hset(&key, WINS_KEY, 0)
                .hset(&key, LOSSES_KEY, 0)
                .hset(&key, DRAWS_KEY, 0)
                .expire(&key, INITIAL_EXPIRE_SECONDS)
                .query_async(&mut redis_conn)
                .await;

            if let Err(e) = pipe_result {
                eprintln!("Redis Error on user init pipe: {e}.");
                // The key was created with hset_nx, but population failed.
                // The partial record will expire eventually.
                // We stop here to prevent having a half-initialized player.
                return;
            }
            final_uuid = Some(uuid);
            break; // Successfully created, exit loop.
        }
        // If not created (collision), the loop continues to the next attempt.
    }

    let uuid = match final_uuid {
        Some(u) => u,
        None => {
            // If we couldn't find a unique UUID after all retries, log it and drop the connection.
            eprintln!(
                "Failed to create a unique user for IP {} after {} retries.",
                addr.ip(),
                MAX_UUID_RETRIES
            );
            return;
        }
    };

    let player = Player::new(uuid, hero, name, addr, ws_stream);
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
