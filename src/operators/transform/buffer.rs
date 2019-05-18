use futures::{Stream, Async, Poll};
use futures::stream::Fuse;
use std::mem;
use std::collections::VecDeque;

#[derive(Debug)]
enum ControlledBufferValue<V> {
    Value(V),
    BufferStart,
    BufferEnd,
}

pub trait Buffer<V> {
    fn insert(&mut self, v: V) -> ();
    fn poll_buffer(&mut self) -> Option<Vec<V>>;
}

#[derive(Default)]
pub struct SimpleCountBuffer<V> {
    vec: Vec<V>,
    max_count: usize,
}
impl<V> SimpleCountBuffer<V> {
    fn new(max_count: usize) -> Self {
        SimpleCountBuffer {
            vec: Vec::with_capacity(max_count),
            max_count: max_count,
        }
    }
}
impl<V> Buffer<V> for SimpleCountBuffer<V> {
    fn insert(&mut self, v:V) -> () {
        self.vec.push(v);
    }
    fn poll_buffer(&mut self) -> Option<Vec<V>> {
        if self.vec.len() == self.max_count {
            return Some(mem::replace(&mut self.vec, Vec::new()))
        } else {
            return None
        }
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct BufferedStream<S: Stream, B: Buffer<S::Item>> {
    s: S,
    buffer: B,
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
                if let Some(r) = self.buffer.poll_buffer() {
                    return Ok(Async::Ready(Some(r)))
                }
                return Ok(Async::Ready(None))
            }
        }
    }
}

pub type SimpleCountBufferedStream<S: Stream> = BufferedStream<S, SimpleCountBuffer<S::Item>>;
impl<S: Stream> SimpleCountBufferedStream<S> {
    pub fn new(s: S, max_count: usize) -> Self {
        SimpleCountBufferedStream {
            s: s,
            buffer: SimpleCountBuffer::new(max_count),
        }
    }
}