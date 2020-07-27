use futures::{Stream};
use futures::stream::Fuse;
use futures::StreamExt;
use futures::task::{Context, Poll};
use pin_project::{pin_project};
use core::pin::Pin;

#[pin_project(project = CombineLatestProj)]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct CombineLatest<S1, S2> 
    where 
        S1: Stream, 
        S1::Item: Clone, 
        S2: Stream,
        S2::Item: Clone
{
    #[pin]
    s1: Fuse<S1>,
    #[pin]
    s2: Fuse<S2>,
    queued1: Option<S1::Item>,
    queued2: Option<S2::Item>,
}

pub fn combine_latest<S1, S2>(s1: S1, s2: S2) -> 
    impl Stream<Item=(S1::Item, S2:: Item)> 
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

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let CombineLatestProj { mut s1, mut s2, queued1, queued2 } = self.project();

        let s1_ready = match s1.as_mut().poll_next(cx) {
            Poll::Ready(Some(item1)) => {
                *queued1 = Some(item1);
                true
            }
            Poll::Ready(None) | Poll::Pending => false
        };

        let s2_ready = match s2.as_mut().poll_next(cx) {
            Poll::Ready(Some(item2)) => {
                *queued2 = Some(item2);
                true
            }
            Poll::Ready(None) | Poll::Pending => false
        };

        if queued1.is_some() && 
            queued2.is_some() && 
            (s1_ready || s2_ready) {
                let pair = (
                    queued1.clone().unwrap(),
                    queued2.clone().unwrap()
                );
                Poll::Ready(Some(pair))
        } else if s1.is_done() && s2.is_done() {
            Poll::Ready(None)
        } else if s1.is_done() && queued1.is_none() ||
            s2.is_done() && queued2.is_none() 
        {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}

#[pin_project(project = CombineLatestVecProj)]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct CombineLatestVec<S: Stream> 
    where S::Item: Clone, 
{
    s_list: Vec<Fuse<S>>,
    queued_list: Vec<Option<S::Item>>,
}

pub fn combine_latest_vec<S>(s: Vec<S>) -> 
    impl Stream<Item=Vec<S::Item>> 
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

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let CombineLatestVecProj { s_list, queued_list } = self.project();
        let mut done_count = 0;
        let len = s_list.len();
        for (i, s) in s_list.iter_mut().enumerate() {
            let has_new = match unsafe{Pin::new_unchecked(s)}.poll_next(cx) {
                Poll::Ready(Some(item1)) => {
                    queued_list[i] = Some(item1);
                    true
                }
                Poll::Ready(None) => {
                    done_count+= 1; 
                    if !queued_list[i].is_some() {
                        // if some stream end but has not yield any value, the entier result done
                        return Poll::Ready(None)
                    }
                    false
                }
                Poll::Pending => false
            };
            // If anyone ready
            if has_new {
                let mut r = Vec::with_capacity(len);
                for q in queued_list {
                    if q.is_some() {
                        r.push(q.clone().unwrap());
                    } else {
                        // If any queued item is not filled, the stream is not ready
                        return Poll::Pending;
                    }
                }
                return Poll::Ready(Some(r));
            }
        }
        // Not a single has_new
        if done_count == len {
            // everyone is done 
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}