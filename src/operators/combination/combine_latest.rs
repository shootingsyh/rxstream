extern crate either;
use either::{Either, Left, Right};
use futures::{Stream, Async, Poll};
use futures::stream::Fuse;


#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct CombineLatest<S1, S2> 
    where 
        S1: Stream, 
        S1::Item: Clone, 
        S2: Stream,
        S2::Item: Clone
{
    s1: Fuse<S1>,
    s2: Fuse<S2>,
    queued1: Option<S1::Item>,
    queued2: Option<S2::Item>,
}

pub fn combine_latest<S1, S2>(s1: S1, s2: S2) -> 
    impl Stream<Item=(S1::Item, S2:: Item), Error=Either<S1::Error, S2::Error>> 
    where 
    S1: Stream, 
    S1::Item: Clone, 
    S2: Stream,
    S2::Item: Clone 
{
    CombineLatest::new(s1, s2)
}

impl<S1, S2> CombineLatest<S1, S2> 
    where 
        S1: Stream, 
        S1::Item: Clone, 
        S2: Stream,
        S2::Item: Clone,  
{
    pub fn new(stream1: S1, stream2: S2) -> CombineLatest<S1, S2>
        where 
            S1: Stream, 
            S1::Item: Clone, 
            S2: Stream,
            S2::Item: Clone
    {
        CombineLatest {
            s1: stream1.fuse(),
            s2: stream2.fuse(),
            queued1: None,
            queued2: None,
        }
    }
}

impl<S1, S2> Stream for CombineLatest<S1, S2>
    where 
        S1: Stream, 
        S1::Item: Clone, 
        S2: Stream,
        S2::Item: Clone, 
{
    type Item = (S1::Item, S2::Item);
    type Error = Either<S1::Error, S2::Error>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let s1_ready = match self.s1.poll() {
            Ok(Async::Ready(Some(item1))) => {
                self.queued1 = Some(item1);
                true
            }
            Ok(Async::Ready(None)) | Ok(Async::NotReady) => false,
            Err(e) => return Err(Left(e))
        };

        let s2_ready = match self.s2.poll() {
            Ok(Async::Ready(Some(item2))) => {
                self.queued2 = Some(item2);
                true
            }
            Ok(Async::Ready(None)) | Ok(Async::NotReady) => false,
            Err(e) => return Err(Right(e))
        };

        if self.queued1.is_some() && 
            self.queued2.is_some() && 
            (s1_ready || s2_ready) {
                let pair = (
                    self.queued1.clone().unwrap(),
                    self.queued2.clone().unwrap()
                );
                Ok(Async::Ready(Some(pair)))
        } else if self.s1.is_done() && self.s2.is_done() {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}