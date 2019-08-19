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
        } else if self.s1.is_done() && self.queued1.is_none() ||
            self.s2.is_done() && self.queued2.is_none() 
        {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct CombineLatestVec<S: Stream> 
    where S::Item: Clone, 
{
    s_list: Vec<Fuse<S>>,
    queued_list: Vec<Option<S::Item>>,
}

pub fn combine_latest_vec<S>(s: Vec<S>) -> 
    impl Stream<Item=Vec<S::Item>, Error=S::Error> 
    where 
    S: Stream, 
    S::Item: Clone, 
{
    CombineLatestVec::new(s)
}

impl<S> CombineLatestVec<S> 
    where 
        S: Stream, 
        S::Item: Clone,   
{
    pub fn new(streams: Vec<S>) -> CombineLatestVec<S>
        where 
            S: Stream, 
            S::Item: Clone, 
    {
        let len = streams.len();
        CombineLatestVec {
            s_list: streams.into_iter().map(|s| s.fuse()).collect(),
            queued_list: vec![None; len],
        }
    }
}

impl<S> Stream for CombineLatestVec<S>
    where 
        S: Stream, 
        S::Item: Clone,  
{
    type Item = Vec<S::Item>;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut done_count = 0;
        let len = self.s_list.len();
        for (i, s) in self.s_list.iter_mut().enumerate() {
            let has_new = match s.poll() {
                Ok(Async::Ready(Some(item1))) => {
                    self.queued_list[i] = Some(item1);
                    true
                }
                Ok(Async::Ready(None)) => {
                    done_count+= 1; 
                    if !self.queued_list[i].is_some() {
                        // if some stream end but has not yield any value, the entier result done
                        return Ok(Async::Ready(None))
                    }
                    false
                },
                Ok(Async::NotReady) => false,
                Err(e) => return Err(e)
            };
            // If anyone ready
            if has_new {
                let mut r = Vec::with_capacity(self.s_list.len());
                for q in &self.queued_list {
                    if q.is_some() {
                        r.push(q.clone().unwrap());
                    } else {
                        // If any queued item is not filled, the stream is not ready
                        return Ok(Async::NotReady);
                    }
                }
                return Ok(Async::Ready(Some(r)))
            }
        }
        // Not a single has_new
        if done_count == len {
            // everyone is done 
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}