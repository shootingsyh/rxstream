use super::buffered_stream::{Buffer, BufferedStream};
use super::simple_count_buffer::SimpleCountBuffer;
use std::mem;
use futures::Stream;
use std::collections::VecDeque;

#[derive(Default)]
pub struct OverlappedCountBuffer<V: Clone> {
    buffers: VecDeque<SimpleCountBuffer<V>>,
    skip: usize,
    max_count: usize,
    skip_count: usize,
}


impl<V: Clone> OverlappedCountBuffer<V> {
    fn new(max_count: usize, skip: usize) -> Self {
        let mut r = OverlappedCountBuffer {
            skip: skip,
            skip_count: 0,
            max_count: max_count,
            buffers: VecDeque::new(),
        };
        r.buffers.push_back(SimpleCountBuffer::new(max_count));
        r
    }
}

impl<V: Clone> Buffer<V> for OverlappedCountBuffer<V> {
    fn insert(&mut self, v:V) -> () {
        for buffer in self.buffers.iter_mut() {
            buffer.insert(v.clone())
        }
        self.skip_count += 1;
        if self.skip_count == self.skip {
            self.buffers.push_back(SimpleCountBuffer::new(self.max_count));
            self.skip_count = 0;
        } 
    }
    fn poll_buffer(&mut self) -> Option<Vec<V>> {
        let result = if let Some(front) = self.buffers.front_mut() {
            if let Some(r) = front.poll_buffer() {
                Some(r)
            } else {
                None
            }
        } else {
            None
        };
        if result.is_some() {
            self.buffers.pop_front();
        }
        result
    }

    fn poll_buffer_after_done(&mut self) -> Option<Vec<V>> {
        if let Some(mut front) = self.buffers.pop_front() {
            // If overlapped buffers still have buffer, return either the poll after done result
            // or empty vec, so we can continue to poll next buffer
            if let Some(r) = front.poll_buffer_after_done() {
                Some(r) 
            } else {
                Some(Vec::new())
            }
        } else {
            None
        }
    }
}


pub type OverlappedCountBufferedStream<S: Stream> where S::Item: Clone = BufferedStream<S, OverlappedCountBuffer<S::Item>>;
impl<S: Stream> OverlappedCountBufferedStream<S> where S::Item: Clone {
    pub fn new(s: S, max_count: usize, skip: usize) -> Self {
        OverlappedCountBufferedStream {
            s: s.fuse(),
            buffer: OverlappedCountBuffer::new(max_count, skip),
        }
    }
}