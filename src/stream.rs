use core::pin::Pin;
use std::task::Context;

use futures::stream::FusedStream;
use futures::{Poll, Stream};

#[derive(Debug)]
pub struct SclutchStream<St1, St2> {
    stream: St1,
    clutch: St2,
    done: bool,
}

impl<St1: Unpin, St2: Unpin> Unpin for SclutchStream<St1, St2> {}

impl<St1, St2> SclutchStream<St1, St2>
where
    St1: Stream,
    St2: Stream,
{
    pub fn new(s1: St1, s2: St2) -> SclutchStream<St1, St2> {
        SclutchStream {
            stream: s1,
            clutch: s2,
            done: false,
        }
    }
}

impl<St1, St2> FusedStream for SclutchStream<St1, St2> {
    fn is_terminated(&self) -> bool {
        self.done
    }
}

impl<St1, St2> Stream for SclutchStream<St1, St2>
where
    St1: Stream,
    St2: Stream,
{
    type Item = St1::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<St1::Item>> {
        if self.is_terminated() {
            return Poll::Ready(None);
        }

        let SclutchStream {
            stream,
            clutch,
            done,
        } = unsafe { Pin::get_unchecked_mut(self) };

        let clutch = unsafe { Pin::new_unchecked(clutch) };
        if let Poll::Ready(_) = clutch.poll_next(cx) {
            *done = true;
            return Poll::Ready(None);
        }

        let stream = unsafe { Pin::new_unchecked(stream) };
        match stream.poll_next(cx) {
            Poll::Ready(None) => {
                *done = true;
                Poll::Ready(None)
            }
            v => v,
        }
    }
}
