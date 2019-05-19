use super::buffered_stream::{Buffer, BufferedStream};
use std::mem;
use futures::Stream;
use std::time::{Instant, Duration};

#[derive(Default)]
pub struct SimpleTimeBuffer<V> {
    vec: Vec<V>,
    start_time: Option<Instant>,
    time_span: Duration,
}
impl<V> SimpleTimeBuffer<V> {
    fn new(time_span: Duration) -> Self {
        SimpleTimeBuffer {
            vec: Vec::new(),
            time_span: time_span,
            start_time: None,
        }
    }
}
impl<V> Buffer<V> for SimpleTimeBuffer<V> {
    fn insert(&mut self, v:V) -> () {
        self.vec.push(v);
    }
    fn poll_buffer(&mut self) -> Option<Vec<V>> {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
        if Instant::now().duration_since(self.start_time.unwrap()) >= self.time_span {
            self.start_time = Some(Instant::now());
            return Some(mem::replace(&mut self.vec, Vec::new()))
        }
        None
    }
}


pub type SimpleTimeBufferredStream<S: Stream> = BufferedStream<S, SimpleTimeBuffer<S::Item>>;
impl<S: Stream> SimpleTimeBufferredStream<S> {
    pub fn new(s: S, time_span: Duration) -> Self {
        SimpleTimeBufferredStream {
            s: s,
            buffer: SimpleTimeBuffer::new(time_span),
        }
    }
}