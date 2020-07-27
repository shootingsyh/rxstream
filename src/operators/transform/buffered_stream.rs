use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::{Stream};
use futures::stream::Fuse;
use pin_project::pin_project;

pub trait Buffer {
    type V;
    fn insert(&mut self, v: Self::V) -> ();
    fn poll_buffer(&mut self, cx: &mut Context) -> Option<Vec<Self::V>>;
    /// A function will only be called once the buffered stream end,
    /// so the buffer can decide whether to return the partially buffered item.
    /// By default it calls to poll_buffer
    fn poll_buffer_after_done(&mut self, cx: &mut Context) -> Option<Vec<Self::V>> {
        return self.poll_buffer(cx);
    }
}

#[pin_project(project=BufferedStreamProj)]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct BufferedStream<S: Stream, B: Buffer<V=S::Item>> {
    #[pin]
    pub s: Fuse<S>,
    pub buffer: B,
}

impl<S, B> Stream for BufferedStream<S,B> where S: Stream, B: Buffer<V=S::Item>  {
    type Item = Vec<S::Item>;

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let BufferedStreamProj {mut s, buffer} = self.project();
        // After stream is done, buffer will be polled with 
        // poll_buffer_after_done. None means end.
        if s.is_done() {
            if let Some(r) = buffer.poll_buffer_after_done(cx) {
                return Poll::Ready(Some(r))
            } 
            return Poll::Ready(None)
        } else {
            // Before stream is end, buffer will be polled with
            // poll and None means not ready.
            loop {
                if let Some(r) = buffer.poll_buffer(cx) {
                    return Poll::Ready(Some(r))
                }
                if let Some(r) = futures::ready!(s.as_mut().poll_next(cx)) {
                    buffer.insert(r);
                } else {
                    // If inner stream ended, try fetch one more time from buffer
                    if let Some(r) = buffer.poll_buffer_after_done(cx) {
                        return Poll::Ready(Some(r))
                    }
                    return Poll::Ready(None)
                }
            }
        }
    }
}