use super::*;

type SocketReader = SplitStream<WebSocketStream<TcpStream>>;
type WriteTaskHandle = JoinHandle<SplitSink<WebSocketStream<TcpStream>, Message>>;

const MAX_MATCHING_TIME: u16 = 15000;

static NEW: SegQueue<UserBlob> = SegQueue::new();

struct UserBlob {
    uuid: Uuid,
    addr: SocketAddr,
    read: SocketReader,
    tx: UnboundedSender<String>,
    write_task: WriteTaskHandle,
    previous_instant: Instant,
    millis: u16,
    state: State,
}

impl UserBlob {
    pub fn new(uuid: Uuid, addr: SocketAddr, ws_stream: WebSocketStream<TcpStream>) -> Self {
        let (tx, mut rx) = unbounded_channel::<String>();
        let (mut write, read) = ws_stream.split();
        let write_task: WriteTaskHandle = tokio::spawn(async move {
            while let Some(s) = rx.recv().await {
                let result = write.send(Message::text(s)).await;
                if let Err(e) = result {
                    eprintln!("Failed to send message to WebSocket (Address:{addr}): {e}");
                    return write;
                }
            }
            write
        });

        Self {
            uuid,
            addr,
            read,
            tx,
            write_task,
            previous_instant: Instant::now(),
            millis: MAX_MATCHING_TIME,
            state: State::Matching,
        }
    }
}

pub async fn update() {
    const TICK: u64 = 1000 / 15;
    const PERIOD: Duration = Duration::from_millis(TICK);
    let mut interval = interval(PERIOD);
    let mut users = VecDeque::new();
    let mut temp = VecDeque::new();
    loop {
        let instant = interval.tick().await;

        while let Some(user) = NEW.pop() {
            users.push_back(user);
        }

        'update: while let Some(mut user) = users.pop_front() {
            let elapsed = instant
                .saturating_duration_since(user.previous_instant)
                .as_millis();
            user.previous_instant = instant;
            user.millis = user.millis.saturating_sub(elapsed as u16);

            #[cfg(not(feature = "no-debuging-log"))]
            println!("Addr:{} - millis: {}", user.addr, user.millis);

            let packet = Packet::MatchingStatus {
                millis: user.millis,
            };
            user.tx
                .send(serde_json::to_string(&packet).unwrap())
                .unwrap();

            loop {
                match poll_stream_nonblocking(&mut user.read) {
                    StreamPollResult::Pending => break,
                    StreamPollResult::Item(message) => {
                        if let Message::Text(s) = message
                            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                        {
                            match packet {
                                Packet::CancelGame => {
                                    user.state = State::Title;

                                    drop(user.tx);
                                    let other = user.write_task.await.unwrap();
                                    let ws_stream = user.read.reunite(other).unwrap();
                                    next_state(user.uuid, user.state, ws_stream, user.addr);
                                    continue 'update;
                                }
                                _ => { /* empty */ }
                            }
                        }
                    }
                    StreamPollResult::Error(e) => {
                        println!("WebSocket disconnected (Address:{}): {e}", user.addr);
                        continue 'update;
                    }
                    StreamPollResult::Closed => {
                        println!("WebSocket disconnected (Address:{})", user.addr);
                        continue 'update;
                    }
                }
            }

            if user.millis == 0 {
                // --- Temp code ---
                drop(user.tx);
                let other = user.write_task.await.unwrap();
                let ws_stream = user.read.reunite(other).unwrap();
                next_state(user.uuid, user.state, ws_stream, user.addr);
                continue 'update;
            }

            temp.push_back(user);
        }

        mem::swap(&mut users, &mut temp);
    }
}

pub async fn regist(uuid: Uuid, addr: SocketAddr, ws_stream: WebSocketStream<TcpStream>) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("Addr:{addr} - Current State: Matching");
    NEW.push(UserBlob::new(uuid, addr, ws_stream));
}
