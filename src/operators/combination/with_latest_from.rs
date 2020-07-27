use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::{Stream, StreamExt};
use futures::stream::Fuse;
use pin_project::pin_project;

#[pin_project(project=WithLatestFromProj)]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct WithLatestFrom<S1, S2> 
    where 
        S1: Stream, 
        S2: Stream,
        S2::Item: Clone
{
    #[pin]
    source: Fuse<S1>,
    #[pin]
    other: Fuse<S2>,
    queued_source: Option<S1::Item>,
    queued_other: Option<S2::Item>,
}

impl<S1, S2> WithLatestFrom<S1, S2> 
    where 
        S1: Stream, 
        S2: Stream,
        S2::Item: Clone,  
{
    pub fn new(stream1: S1, stream2: S2) -> WithLatestFrom<S1, S2>
        where 
            S1: Stream, 
            S2: Stream,
            S2::Item: Clone
    {
        WithLatestFrom {
            source: stream1.fuse(),
            other: stream2.fuse(),
            queued_source: None,
            queued_other: None,
        }
    }
}

impl<S1, S2> Stream for WithLatestFrom<S1, S2>
    where 
        S1: Stream, 
        S2: Stream,
        S2::Item: Clone, 
{
    type Item = (S1::Item, S2::Item);

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let WithLatestFromProj { mut source, mut other, queued_source, queued_other} = self.project();
        let source_ready = match source.as_mut().poll_next(cx) {
            Poll::Ready(Some(item1)) => {
                *queued_source = Some(item1);
                true
            }
            Poll::Ready(None) | Poll::Pending => false
        };

        match other.as_mut().poll_next(cx) {
            Poll::Ready(Some(item2)) => {
                *queued_other = Some(item2);
            }
            Poll::Ready(None) | Poll::Pending => ()
        };
        // End the stream if source is done, any time
        // Or if other is done before emit anything
        if source.is_done() || other.is_done() && queued_other.is_none() {
            Poll::Ready(None)
        } 
        else if source_ready {
            if queued_other.is_some() {
                Poll::Ready(Some((queued_source.take().unwrap(), queued_other.clone().unwrap())))
            } else {
                Poll::Pending
            }
        } else {
            Poll::Pending
        }
    }
}