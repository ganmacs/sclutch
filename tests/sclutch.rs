use futures::{self, future, task, FutureExt, Poll, StreamExt};
use sclutch;

pub fn noop_context() -> task::Context<'static> {
    task::Context::from_waker(futures::task::noop_waker_ref())
}

#[test]
fn trigger_off() {
    let mut cx = noop_context();
    let s = futures::stream::iter(vec![1, 2, 3]);
    let (trigger, mut ss) = sclutch::new(s);
    assert_eq!(ss.poll_next_unpin(&mut cx), Poll::Ready(Some(1)));
    assert_eq!(ss.poll_next_unpin(&mut cx), Poll::Ready(Some(2)));
    assert_eq!(ss.poll_next_unpin(&mut cx), Poll::Ready(Some(3)));
}

#[test]
fn trigger_on() {
    let mut cx = noop_context();
    let s = futures::stream::iter(vec![1, 2, 3]);
    let (trigger, mut ss) = sclutch::new(s);
    assert_eq!(ss.poll_next_unpin(&mut cx), Poll::Ready(Some(1)));
    trigger.on();
    assert_eq!(ss.poll_next_unpin(&mut cx), Poll::Ready(None));
    assert_eq!(ss.poll_next_unpin(&mut cx), Poll::Ready(None));
}
