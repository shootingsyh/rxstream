use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::{Stream, StreamExt};
use futures::stream::Fuse;
use std::mem;
use pin_project::pin_project;

#[pin_project(project=PairwiseProj)]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Pairwise<S> where S: Stream, S::Item: Clone {
    #[pin]
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

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let PairwiseProj {mut s, previous} = self.project();
        if previous.is_none() {
            let inner = futures::ready!(s.as_mut().poll_next(cx));
            match inner {
                Some(item) => *previous = Some(item),
                None => return Poll::Ready(None)
            }
        }
        let inner2 = ready!(s.poll_next(cx));
        match inner2 {
            Some(item) => {
                let current = item.clone();
                return Poll::Ready(Some(
                    (
                        mem::replace(previous, Some(item)).unwrap(), 
                        current
                    )
                ));
            },
            None => return Poll::Ready(None)
        }
    }
}