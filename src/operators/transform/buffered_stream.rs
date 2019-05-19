use futures::{Stream, Async, Poll};

pub trait Buffer<V> {
    fn insert(&mut self, v: V) -> ();
    fn poll_buffer(&mut self) -> Option<Vec<V>>;
    /// A function will only be called once the buffered stream end,
    /// so the buffer can decide whether to return the partially buffered item.
    /// By default it calls to poll_buffer
    fn last_poll_buffer(&mut self) -> Option<Vec<V>> {
        return self.poll_buffer();
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct BufferedStream<S: Stream, B: Buffer<S::Item>> {
    pub s: S,
    pub buffer: B,
}

impl<S, B> Stream for BufferedStream<S,B> where S: Stream, B: Buffer<S::Item>  {
    type Item = Vec<S::Item>;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            if let Some(r) = self.buffer.poll_buffer() {
                return Ok(Async::Ready(Some(r)))
            }
            if let Some(r) = futures::try_ready!(self.s.poll()) {
                self.buffer.insert(r);
            } else {
                // If inner stream ended, try fetch one last time from buffer
                if let Some(r) = self.buffer.last_poll_buffer() {
                    return Ok(Async::Ready(Some(r)))
                }
                return Ok(Async::Ready(None))
            }
        }
    }
}