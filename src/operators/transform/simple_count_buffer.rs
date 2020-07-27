use futures::task::Context;
use super::buffered_stream::{Buffer, BufferedStream};
use std::mem;
use futures::{Stream, StreamExt};

#[derive(Default)]
pub struct SimpleCountBuffer<V> {
    vec: Vec<V>,
    max_count: usize,
}
impl<V> SimpleCountBuffer<V> {
    pub fn new(max_count: usize) -> Self {
        SimpleCountBuffer {
            vec: Vec::with_capacity(max_count),
            max_count: max_count,
        }
    }
}
impl<V> Buffer for SimpleCountBuffer<V> {
    type V = V;
    fn insert(&mut self, v:V) -> () {
        self.vec.push(v);
    }
    fn poll_buffer(&mut self, _cx: &mut Context) -> Option<Vec<V>> {
        if self.vec.len() == self.max_count {
            return Some(mem::replace(&mut self.vec, Vec::new()))
        } else {
            return None
        }
    }
    fn poll_buffer_after_done(&mut self, _cx: &mut Context) -> Option<Vec<V>> {
        if self.vec.len() > 0 {
            Some(mem::replace(&mut self.vec, Vec::new()))
        } else {
            None
        }
    }
}


pub type SimpleCountBufferedStream<S> = BufferedStream<S, SimpleCountBuffer<<S as Stream>::Item>>;
impl<S: Stream> SimpleCountBufferedStream<S> {
    pub fn new(s: S, max_count: usize) -> Self {
        SimpleCountBufferedStream {
            s: s.fuse(),
            buffer: SimpleCountBuffer::new(max_count),
        }
    }
}