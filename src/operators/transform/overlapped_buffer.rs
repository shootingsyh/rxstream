use super::buffered_stream::{Buffer, BufferedStream};
use super::simple_count_buffer::SimpleCountBuffer;
use futures::Stream;
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
    buffers: VecDeque<B>,
    opener: O,
    creator: C
}

impl<B: Buffer, O: BufferOpener, C: BufferCreator<B::V, B=B>> OverlappedBuffer<B, O, C> where B::V: Clone {
    fn new_internal(opener: O, creator: C) -> Self {
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
#[derive(Default)] 
pub struct CountBufferOpener {
    skip: usize,
    skip_count: usize,
}
pub struct CountBufferCreator {
    max_count: usize,
}
impl BufferOpener for CountBufferOpener {
    fn check_open(&mut self) -> bool {
        self.skip_count += 1;
        if self.skip_count == self.skip {
            self.skip_count = 0;
            true
        } else {
            false
        }
    }
}
impl<V: Clone> BufferCreator<V> for CountBufferCreator {
    type B = SimpleCountBuffer<V>;
    fn new_buffer(&mut self) -> SimpleCountBuffer<V> {
        SimpleCountBuffer::new(self.max_count)
    }
}

pub type OverlappedCountBuffer<V> = OverlappedBuffer<SimpleCountBuffer<V>, CountBufferOpener, CountBufferCreator>;

impl<V: Clone> OverlappedCountBuffer<V> {
    fn new(max_count: usize, skip: usize) -> Self {
        let mut r = OverlappedCountBuffer::new_internal(CountBufferOpener {
            skip: skip,
            skip_count: 0,
        }, CountBufferCreator {
            max_count: max_count,
        });
        let b = <CountBufferCreator as BufferCreator<V>>::new_buffer(&mut r.creator);
        r.buffers.push_back(b);
        r
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