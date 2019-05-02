use futures::stream::Chain;
use futures::{Future, Stream, Async, Poll};
use either::{Either, Left, Right};

pub fn concat<S1: Stream, S2: Stream<Item=S1::Item, Error=S1::Error>>(s1: S1, s2: S2) -> Chain<S1, S2> {
    s1.chain(s2)
}

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct ChainVec<S: Stream> {
    s_vec: Vec<S>,
    current: usize,
}

pub fn concat_vec<S: Stream>(s_vec: Vec<S>) -> ChainVec<S> {
    ChainVec { s_vec: s_vec, current: 0}    
}

pub fn concat_all<SInner: Stream, SOuter: Stream<Item=SInner>>(s: SOuter) -> 
    impl Stream<Item=SInner::Item, Error=Either<SInner::Error, SOuter::Error>> 
{
    s.collect()
        .map(|streams| concat_vec(streams).map_err(|e| Left(e)))
        .map_err(|e| Right(e))
        .flatten_stream() 
}

impl<S: Stream> Stream for ChainVec<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let len = self.s_vec.len();
        if len == 0 {
            return Ok(Async::Ready(None))
        }
        loop {
            let current_s = &mut self.s_vec[self.current];
            match current_s.poll() {
                Ok(Async::Ready(None)) => {
                    self.current += 1;
                    if len == self.current {
                        return Ok(Async::Ready(None))
                    }
                    ()
                }
                x => return x
            }
        }
    }
}
