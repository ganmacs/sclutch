mod clutch;
mod stream;

pub use clutch::Trigger;
use futures::Stream;

pub type Sclutch<S> = stream::SclutchStream<S, clutch::Clutch>;

pub fn new<S>(stream: S) -> (Trigger, Sclutch<S>)
where
    S: Stream + Unpin,
{
    let (tr, c) = clutch::Clutch::new();
    (tr, stream::SclutchStream::new(stream, c))
}
