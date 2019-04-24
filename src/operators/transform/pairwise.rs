use futures::{Stream, Async, Poll};
use futures::stream::Fuse;
use std::mem;

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Pairwise<S> where S: Stream, S::Item: Clone {
    s: Fuse<S>,
    previous: Option<S::Item>,
}

impl<S> Pairwise<S> where S: Stream, S::Item: Clone {
    pub fn new(s: S) -> Pairwise<S> where S: Stream, S::Item: Clone {
        Pairwise {
            s: s.fuse(),
            previous: None,
        }
    }
} 

impl<S> Stream for Pairwise<S> where S: Stream, S::Item: Clone {
    type Item = (<S as Stream>::Item, <S as Stream>::Item);
    type Error = <S as Stream>::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.previous.is_none() {
            let inner = futures::try_ready!(self.s.poll());
            match inner {
                Some(item) => self.previous = Some(item),
                None => return Ok(Async::Ready(None))
            }
        }
        let inner2 = try_ready!(self.s.poll());
        match inner2 {
            Some(item) => {
                let current = item.clone();
                return Ok(Async::Ready(Some(
                    (
                        mem::replace(&mut self.previous, Some(item)).unwrap(), 
                        current
                    )
                )));
            },
            None => return Ok(Async::Ready(None))
        }
    }
}