use super::overlapped_buffer::{BufferCreator, BufferOpener, OverlappedBuffer};
use super::buffered_stream::BufferedStream;
use super::simple_count_buffer::SimpleCountBuffer;
use futures::Stream;

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

pub type OverlappedCountBufferedStream<S> = BufferedStream<S, OverlappedCountBuffer<<S as Stream>::Item>>;
impl<S: Stream> OverlappedCountBufferedStream<S> where S::Item: Clone {
    pub fn new(s: S, max_count: usize, skip: usize) -> Self {
        OverlappedCountBufferedStream {
            s: s.fuse(),
            buffer: OverlappedCountBuffer::new(max_count, skip),
        }
    }
}