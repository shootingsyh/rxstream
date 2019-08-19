use super::buffered_stream::{Buffer, BufferedStream};
use futures::{Stream, Async};
use std::mem;
use super::super::source;

#[derive(Default)]
pub struct StreamControlledBuffer<V, S: Stream> {
    vec: Vec<V>,
    s: S,
}

impl<V, S: Stream> Buffer for StreamControlledBuffer<V, S> {
    type V = V;
    fn insert(&mut self, v:V) -> () {
        self.vec.push(v);
    }
    fn poll_buffer(&mut self) -> Option<Vec<V>> {
        match self.s.poll() {
            Ok(Async::Ready(_)) => Some(mem::replace(&mut self.vec, Vec::new())),
            Ok(Async::NotReady) => None,
            _ => None
        }
    }
}

pub type SimpleTimeBuffer<V> = StreamControlledBuffer<V, source::TimerStream>;
pub type SimpleTimeBufferredStream<S> = BufferedStream<S, SimpleTimeBuffer<<S as Stream>::Item>>;

pub fn new_simple_time_buffer<V>(time_span: u64) -> SimpleTimeBuffer<V> {
    StreamControlledBuffer {
        vec: Vec::<V>::new(),
        s: source::interval(time_span),
    }
}

pub fn new_simple_buferred_stream<S: Stream>(s: S, time_span: u64) -> SimpleTimeBufferredStream<S> {
    BufferedStream {
        s: s.fuse(),
        buffer: new_simple_time_buffer(time_span),
    }
}

impl<S: Stream> SimpleTimeBufferredStream<S> {
    pub fn new(s: S, time_span: u64) -> Self {
        SimpleTimeBufferredStream {
            s: s.fuse(),
            buffer: StreamControlledBuffer {
                vec: vec![],
                s: source::interval(time_span)
            },
        }
    }
}