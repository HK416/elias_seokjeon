use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{Stream, task};

pub enum StreamPollResult<T, E> {
    Pending,
    Item(T),
    Error(E),
    Closed,
}

pub fn poll_stream_nonblocking<S, T, E>(stream: &mut S) -> StreamPollResult<T, E>
where
    S: Stream<Item = Result<T, E>> + Unpin,
{
    let waker = task::noop_waker(); // tokio 런타임 없어도 동작
    let mut cx = Context::from_waker(&waker);
    let mut pinned = Pin::new(stream);

    match pinned.as_mut().poll_next(&mut cx) {
        Poll::Ready(Some(Ok(item))) => StreamPollResult::Item(item),
        Poll::Ready(Some(Err(e))) => StreamPollResult::Error(e),
        Poll::Ready(None) => StreamPollResult::Closed,
        Poll::Pending => StreamPollResult::Pending,
    }
}
