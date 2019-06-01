use std::pin::Pin;
use std::task::Context;

use futures::{channel, Future, Poll, Stream};

pub struct Clutch(channel::oneshot::Receiver<()>);

impl Clutch {
    pub fn new() -> (Trigger, Self) {
        let (tx, rx) = channel::oneshot::channel();
        (Trigger(tx), Clutch(rx))
    }
}

impl Stream for Clutch {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll(cx) {
            Poll::Ready(_) => Poll::Ready(None),
            _ => Poll::Pending,
        }
    }
}

pub struct Trigger(channel::oneshot::Sender<()>);

impl Trigger {
    pub fn on(self) {
        self.0.send(()).unwrap()
    }
}
