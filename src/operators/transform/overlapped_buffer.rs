use super::buffered_stream::Buffer;
use std::collections::VecDeque;

pub trait BufferOpener {
    fn check_open(&mut self) -> bool;
}

pub trait BufferCreator<V> {
    type B: Buffer<V=V>;
    fn new_buffer(&mut self) -> Self::B;
}

#[derive(Default)]
pub struct OverlappedBuffer<B, O: BufferOpener, C: BufferCreator<B::V, B=B>> where B: Buffer, B::V: Clone {
    pub buffers: VecDeque<B>,
    pub opener: O,
    pub creator: C
}

impl<B: Buffer, O: BufferOpener, C: BufferCreator<B::V, B=B>> OverlappedBuffer<B, O, C> where B::V: Clone {
    pub fn new_internal(opener: O, creator: C) -> Self {
        OverlappedBuffer {
            buffers: VecDeque::new(),
            opener: opener,
            creator: creator,
        }
    }
}

impl<B: Buffer, O: BufferOpener, C: BufferCreator<B::V, B=B>> Buffer for OverlappedBuffer<B, O, C> where B::V: Clone {
    type V = B::V;

    fn insert(&mut self, v:Self::V) -> () {
        for buffer in self.buffers.iter_mut() {
            buffer.insert(v.clone())
        }
        if self.opener.check_open() {
            self.buffers.push_back(self.creator.new_buffer())
        }
    }
    fn poll_buffer(&mut self) -> Option<Vec<Self::V>> {
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

    fn poll_buffer_after_done(&mut self) -> Option<Vec<Self::V>> {
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