use super::*;

pub async fn update(mut player: Box<Player>, mut redis_conn: MultiplexedConnection) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("{:?} - Current State: Title", player);

    while let Some(result) = player.read.next().await {
        let message = match result {
            Ok(message) => message,
            Err(e) => {
                println!("WebSocket disconnected ({:?}): {}", &player, e);
                return;
            }
        };

        if let Message::Text(s) = message
            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
        {
            match packet {
                Packet::QueryRanking => {
                    let my_uuid = player.uuid.to_string();
                    let result = get_leaderboard_and_my_rank(&mut redis_conn, &my_uuid).await;
                    match result {
                        Ok(packet) => {
                            let result = player.tx.send(packet);
                            if let Err(e) = result {
                                eprintln!("WebSocket disconnected ({:?}): {}", &player, e);
                                return;
                            }
                        }
                        Err(e) => {
                            eprintln!("Redis Error: {e}");
                            return;
                        }
                    }
                }
                Packet::EnterGame => {
                    return next_state(State::Matching, player, redis_conn);
                }
                _ => { /* empty */ }
            }
        }
    }
}

pub async fn get_leaderboard_and_my_rank(
    redis_conn: &mut MultiplexedConnection,
    my_uuid: &str,
) -> redis::RedisResult<Packet> {
    let (top_uuids, my_rank_idx): (Vec<String>, Option<u32>) = redis::pipe()
        .zrevrange(LEADER_BOARD_KEY, 0, 9)
        .zrevrank(LEADER_BOARD_KEY, my_uuid)
        .query_async(redis_conn)
        .await?;

    let my_rank = my_rank_idx.map(|r| r + 1);

    if top_uuids.is_empty() {
        return Ok(Packet::RankingResult {
            my_rank,
            top_list: Vec::new(),
        });
    }

    let mut pipe = redis::pipe();
    for uuid in &top_uuids {
        let key = format!("user:{uuid}");
        pipe.hmget(&key, &[NAME_KEY, WINS_KEY, LOSSES_KEY]);
    }

    let details: Vec<(String, u32, u32)> = pipe.query_async(redis_conn).await?;
    let mut top_list = Vec::with_capacity(top_uuids.len());
    for (i, ((name, wins, losses), uuid)) in details.into_iter().zip(top_uuids).enumerate() {
        top_list.push(RankItem {
            rank: i as u32 + 1,
            uuid,
            name,
            wins,
            losses,
        });
    }

    Ok(Packet::RankingResult { my_rank, top_list })
}
