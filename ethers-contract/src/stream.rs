use ethers_core::types::Log;
use ethers_providers::Id;
use futures_util::stream::{Stream, StreamExt};
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

type MapEvent<'a, R, E> = Box<dyn Fn(Log) -> Result<R, E> + Send + 'a>;

#[pin_project]
/// Generic wrapper around Log streams, mapping their content to a specific
/// deserialized log struct.
///
/// We use this wrapper type instead of `StreamExt::map` in order to preserve
/// information about the filter/subscription's id.
pub struct EventStream<'a, T, R, E> {
    pub id: Id,
    #[pin]
    stream: T,
    parse: MapEvent<'a, R, E>,
}

impl<'a, T, R, E> EventStream<'a, T, R, E> {
    pub fn new(id: impl Into<Id>, stream: T, parse: MapEvent<'a, R, E>) -> Self {
        Self {
            id: id.into(),
            stream,
            parse,
        }
    }
}

impl<'a, T, R, E> Stream for EventStream<'a, T, R, E>
where
    T: Stream<Item = Log> + Unpin,
{
    type Item = Result<R, E>;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        match futures_util::ready!(this.stream.poll_next_unpin(ctx)) {
            Some(item) => Poll::Ready(Some((this.parse)(item))),
            None => Poll::Pending,
        }
    }
}
