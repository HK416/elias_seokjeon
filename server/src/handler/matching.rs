use super::*;

const MAX_MATCHING_TIME: u16 = 15000;
const MAX_LOOP: usize = 100;

static NEW: SegQueue<Node> = SegQueue::new();

struct Node {
    session: Session,
    previous_instant: Instant,
    millis: u16,
}

impl Node {
    pub fn new(session: Session) -> Self {
        Self {
            session,
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
    let mut interval = time::interval(PERIOD);
    let mut nodes = VecDeque::new();
    let mut temp = VecDeque::new();
    loop {
        let instant = interval.tick().await;

        // 1. Move new sessions from the global queue to the local queue.
        while let Some(n) = NEW.pop() {
            #[cfg(not(feature = "no-debugging-log"))]
            println!(
                "Added Matching Queue ({:?}) - Queue Size: {}",
                n.session,
                nodes.len() + 1
            );
            nodes.push_back(n);
        }

        // 2. Poll events for all waiting players before attempting to match.
        // This ensures that cancellation requests are processed with priority.
        'update: while let Some(mut node) = nodes.pop_front() {
            let mut cnt = MAX_LOOP;
            while cnt > 0 {
                match poll_stream_nonblocking(&mut node.session.read) {
                    StreamPollResult::Pending => break,
                    StreamPollResult::Item(message) => {
                        if let Message::Text(s) = message
                            && let Ok(packet) = serde_json::from_str::<Packet>(&s)
                        {
                            match packet {
                                Packet::TryCancelGame => {
                                    node.session.tx.send(Packet::CancelSuccess).unwrap();
                                    next_state(State::Title, node.session);
                                    continue 'update; // Session is removed from matching.
                                }
                                _ => { /* empty */ }
                            }
                        }
                    }
                    StreamPollResult::Error(e) => {
                        println!("WebSocket disconnected ({:?}): {e}", node.session);
                        #[cfg(not(feature = "no-debugging-log"))]
                        println!("Queue Size: {}", nodes.len());
                        continue 'update; // Session is removed due to error.
                    }
                    StreamPollResult::Closed => {
                        println!("WebSocket disconnected ({:?})", node.session);
                        #[cfg(not(feature = "no-debugging-log"))]
                        println!("Queue Size: {}", nodes.len());
                        continue 'update; // Session is removed due to closure.
                    }
                }
                cnt -= 1;
            }
            temp.push_back(node);
        }
        mem::swap(&mut nodes, &mut temp);

        // 3. Try to match sessions who are still in the queue.
        while nodes.len() >= 2 {
            let left = nodes.pop_front().unwrap().session;
            let right = nodes.pop_front().unwrap().session;

            #[cfg(not(feature = "no-debugging-log"))]
            println!("[{:?} VS {:?}] - Queue Size: {}", left, right, nodes.len());

            tokio::spawn(sync::wait(left, right));
        }

        // 4. Update status for the remaining sessions.
        while let Some(mut node) = nodes.pop_front() {
            let elapsed = instant
                .saturating_duration_since(node.previous_instant)
                .as_millis();
            node.previous_instant = instant;
            node.millis = node.millis.saturating_sub(elapsed as u16);

            if node.millis == 0 {
                // --- Temp code ---
                #[cfg(not(feature = "no-debugging-log"))]
                println!(
                    "FIXME: Single player mode is not implemented yet. ({}/{})",
                    file!(),
                    line!()
                );

                next_state(State::Matching, node.session);
                continue;
                //-------------------
            }

            node.session
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

pub async fn regist(session: Session) {
    #[cfg(not(feature = "no-debugging-log"))]
    println!("{:?} - Current State: Matching", session);
    NEW.push(Node::new(session));
}
