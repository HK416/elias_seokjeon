use super::*;

const MAX_MATCHING_TIME: u16 = 15000;
const MAX_LOOP: usize = 100;

static NEW: SegQueue<Node> = SegQueue::new();

struct Node {
    player: Player,
    previous_instant: Instant,
    millis: u16,
}

impl Node {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            previous_instant: Instant::now(),
            millis: MAX_MATCHING_TIME,
        }
    }
}

pub async fn update() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, MemOrdering::AcqRel);
    assert!(n < 1, "This function must be called only once!");
    update_internal().await;
}

async fn update_internal() {
    const TICK: u64 = 1000 / 15;
    const PERIOD: Duration = Duration::from_millis(TICK);
    let mut interval = interval(PERIOD);
    let mut nodes = VecDeque::new();
    let mut temp = VecDeque::new();
    loop {
        let instant = interval.tick().await;

        // 1. Move new players from the global queue to the local queue.
        while let Some(n) = NEW.pop() {
            #[cfg(not(feature = "no-debuging-log"))]
            println!(
                "Added Matching Queue ({:?}) - Queue Size: {}",
                n.player,
                nodes.len() + 1
            );
            nodes.push_back(n);
        }

        // 2. Poll events for all waiting players before attempting to match.
        // This ensures that cancellation requests are processed with priority.
        'update: while let Some(mut node) = nodes.pop_front() {
            let mut cnt = MAX_LOOP;
            while cnt > 0 {
                match poll_stream_nonblocking(&mut node.player.read) {
                    StreamPollResult::Pending => break,
                    StreamPollResult::Item(message) => {
                        if let Message::Text(s) = message
                            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                        {
                            match packet {
                                Packet::TryCancelGame => {
                                    node.player.tx.send(Packet::CancelSuccess).unwrap();
                                    next_state(State::Title, node.player);
                                    continue 'update; // Player is removed from matching.
                                }
                                _ => { /* empty */ }
                            }
                        }
                    }
                    StreamPollResult::Error(e) => {
                        println!("WebSocket disconnected ({:?}): {e}", node.player);
                        continue 'update; // Player is removed due to error.
                    }
                    StreamPollResult::Closed => {
                        println!("WebSocket disconnected ({:?})", node.player);
                        continue 'update; // Player is removed due to closure.
                    }
                }
                cnt -= 1;
            }
            temp.push_back(node);
        }
        mem::swap(&mut nodes, &mut temp);

        // 3. Try to match players who are still in the queue.
        while nodes.len() >= 2 {
            let p0 = nodes.pop_front().unwrap().player;
            let p1 = nodes.pop_front().unwrap().player;

            #[cfg(not(feature = "no-debuging-log"))]
            println!("[{:?} VS {:?}] - Current State: Matching", p0, p1);

            p0.tx
                .send(Packet::MatchingSuccess {
                    other: p1.name.clone(),
                    hero: p1.hero,
                })
                .unwrap();
            p1.tx
                .send(Packet::MatchingSuccess {
                    other: p0.name.clone(),
                    hero: p0.hero,
                })
                .unwrap();
        }

        // 4. Update status for the remaining players.
        while let Some(mut node) = nodes.pop_front() {
            let elapsed = instant
                .saturating_duration_since(node.previous_instant)
                .as_millis();
            node.previous_instant = instant;
            node.millis = node.millis.saturating_sub(elapsed as u16);

            if node.millis == 0 {
                // --- Temp code ---
                next_state(State::Matching, node.player);
                continue;
                //-------------------
            }

            node.player
                .tx
                .send(Packet::MatchingStatus {
                    millis: node.millis,
                })
                .unwrap();

            temp.push_back(node);
        }

        mem::swap(&mut nodes, &mut temp);
    }
}

pub async fn regist(player: Player) {
    #[cfg(not(feature = "no-debuging-log"))]
    println!("{:?} - Current State: Matching", player);
    NEW.push(Node::new(player));
}
