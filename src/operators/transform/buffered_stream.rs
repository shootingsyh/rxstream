use futures::{Stream, Async, Poll};
use futures::stream::Fuse;

pub trait Buffer {
    type V;
    fn insert(&mut self, v: Self::V) -> ();
    fn poll_buffer(&mut self) -> Option<Vec<Self::V>>;
    /// A function will only be called once the buffered stream end,
    /// so the buffer can decide whether to return the partially buffered item.
    /// By default it calls to poll_buffer
    fn poll_buffer_after_done(&mut self) -> Option<Vec<Self::V>> {
        return self.poll_buffer();
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct BufferedStream<S: Stream, B: Buffer<V=S::Item>> {
    pub s: Fuse<S>,
    pub buffer: B,
}

impl<S, B> Stream for BufferedStream<S,B> where S: Stream, B: Buffer<V=S::Item>  {
    type Item = Vec<S::Item>;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // After stream is done, buffer will be polled with 
        // poll_buffer_after_done. None means end.
        if self.s.is_done() {
            if let Some(r) = self.buffer.poll_buffer_after_done() {
                return Ok(Async::Ready(Some(r)))
            } 
            return Ok(Async::Ready(None))
        } else {
            // Before stream is end, buffer will be polled with
            // poll and None means not ready.
            loop {
                if let Some(r) = self.buffer.poll_buffer() {
                    return Ok(Async::Ready(Some(r)))
                }
                if let Some(r) = futures::try_ready!(self.s.poll()) {
                    self.buffer.insert(r);
                } else {
                    // If inner stream ended, try fetch one more time from buffer
                    if let Some(r) = self.buffer.poll_buffer_after_done() {
                        return Ok(Async::Ready(Some(r)))
                    }
                    return Ok(Async::Ready(None))
                }
            }
        }
    }
}