use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::stream::Chain;
use futures::{StreamExt, FutureExt, Stream};
use pin_project::pin_project;

pub fn concat<S1: Stream, S2: Stream<Item=S1::Item>>(s1: S1, s2: S2) -> Chain<S1, S2> {
    s1.chain(s2)
}

#[pin_project(project=ChainVecProj)]
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
    impl Stream<Item=SInner::Item> 
{
    s.collect()
        .map(|streams| concat_vec(streams))
        .flatten_stream() 
}

impl<S: Stream> Stream for ChainVec<S> {
    type Item = S::Item;

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let ChainVecProj { s_vec, current } = self.project();
        let len = s_vec.len();
        if len == 0 {
            return Poll::Ready(None)
        }
        loop {
            let current_s = unsafe {Pin::new_unchecked(&mut s_vec[*current])};
            match current_s.poll_next(cx) {
                Poll::Ready(None) => {
                    *current += 1;
                    if len == *current {
                        return Poll::Ready(None)
                    }
                    ()
                }
                x => return x
            }
        }
    }
}
