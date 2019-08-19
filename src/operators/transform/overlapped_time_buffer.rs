use super::overlapped_buffer::{BufferCreator, BufferOpener, OverlappedBuffer};
use super::buffered_stream::BufferedStream;
use super::simple_time_buffer::{new_simple_time_buffer, SimpleTimeBuffer};
use std::time::{Duration, Instant};
use futures::{Stream};


pub struct TimeBufferOpener {
    period: u64,
    last_check: Instant,
}
pub struct TimeBufferCreator {
    time_span: u64,
}

impl TimeBufferOpener {
    fn new(period: u64) -> Self {
        Self {
            period: period,
            last_check: Instant::now()
        }
    }
}

impl BufferOpener for TimeBufferOpener {
    fn check_open(&mut self) -> bool {
        let now = Instant::now();
        if now >= self.last_check + Duration::from_millis(self.period) {
            self.last_check = now;
            true
        } else {
            false
        }
    }
}

impl<V: Clone> BufferCreator<SimpleTimeBuffer<V>> for TimeBufferCreator {
    fn new_buffer(&mut self) -> SimpleTimeBuffer<V> {
        new_simple_time_buffer(self.time_span)
    }
}

pub type OverlappedTimeBuffer<V> = OverlappedBuffer<SimpleTimeBuffer<V>, TimeBufferOpener, TimeBufferCreator>;

impl<V: Clone> OverlappedTimeBuffer<V> {
    fn new(time_span: u64, creation_interval: u64) -> Self {
        let mut r = OverlappedTimeBuffer::new_internal(TimeBufferOpener::new(creation_interval), TimeBufferCreator {
            time_span: time_span,
        });
        let b = <TimeBufferCreator as BufferCreator<SimpleTimeBuffer<V>>>::new_buffer(&mut r.creator);
        r.buffers.push_back(b);
        r
    } 
}

pub type OverlappedTimeBufferedStream<S> = BufferedStream<S, OverlappedTimeBuffer<<S as Stream>::Item>>;
impl<S: Stream> OverlappedTimeBufferedStream<S> where S::Item: Clone {
    pub fn new(s: S, time_span: u64, creation_interval: u64) -> Self {
        OverlappedTimeBufferedStream {
            s: s.fuse(),
            buffer: OverlappedTimeBuffer::new(time_span, creation_interval),
        }
    }
}