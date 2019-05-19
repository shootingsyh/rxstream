use super::buffered_stream::{Buffer, BufferedStream};
use std::mem;
use futures::Stream;

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
    fn last_poll_buffer(&mut self) -> Option<Vec<V>> {
        if self.vec.len() > 0 {
            Some(mem::replace(&mut self.vec, Vec::new()))
        } else {
            None
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