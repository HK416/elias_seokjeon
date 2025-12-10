use super::*;

pub async fn setup(addr: SocketAddr, ws_stream: WebSocketStream<TcpStream>) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("Addr:{addr} - Current State: Init");

    let uuid = Uuid::new_v4();
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

    next_state(State::Title, Box::new(player));
}
