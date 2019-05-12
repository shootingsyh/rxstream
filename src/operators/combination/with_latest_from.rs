extern crate either;
use either::{Either, Left, Right};
use futures::{Stream, Async, Poll};
use futures::stream::Fuse;


#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct WithLatestFrom<S1, S2> 
    where 
        S1: Stream, 
        S2: Stream,
        S2::Item: Clone
{
    source: Fuse<S1>,
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
    type Error = Either<S1::Error, S2::Error>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let source_ready = match self.source.poll() {
            Ok(Async::Ready(Some(item1))) => {
                self.queued_source = Some(item1);
                true
            }
            Ok(Async::Ready(None)) | Ok(Async::NotReady) => false,
            Err(e) => return Err(Left(e))
        };

        match self.other.poll() {
            Ok(Async::Ready(Some(item2))) => {
                self.queued_other = Some(item2);
            }
            Ok(Async::Ready(None)) | Ok(Async::NotReady) => (),
            Err(e) => return Err(Right(e))
        };
        // End the stream if source is done, any time
        // Or if other is done before emit anything
        if self.source.is_done() || self.other.is_done() && self.queued_other.is_none() {
            Ok(Async::Ready(None))
        } 
        else if source_ready {
            if self.queued_other.is_some() {
                Ok(Async::Ready(Some((self.queued_source.take().unwrap(), self.queued_other.clone().unwrap()))))
            } else {
                Ok(Async::NotReady)
            }
        } else {
            Ok(Async::NotReady)
        }
    }
}