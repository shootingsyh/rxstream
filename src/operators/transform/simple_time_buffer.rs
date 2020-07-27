use futures::task::Context;
use futures::task::Poll;
use super::buffered_stream::{Buffer, BufferedStream};
use futures::{Stream, StreamExt};
use std::mem;
use super::super::source;
use std::pin::Pin;
use std::ops::{Deref, DerefMut};

pub struct StreamControlledBuffer<V, D> {
    vec: Vec<V>,
    s: Pin<D>,
}

impl<V, D> Buffer for StreamControlledBuffer<V, D> where D: DerefMut, <D as Deref>::Target: Stream {
    type V = V;
    fn insert(&mut self, v:V) -> () {
        self.vec.push(v);
    }
    fn poll_buffer(&mut self, cx: &mut Context<'_>) -> Option<Vec<V>> {
        match self.s.as_mut().poll_next(cx) {
            Poll::Ready(_) => Some(mem::replace(&mut self.vec, Vec::new())),
            Poll::Pending => None
        }
    }
}

pub type SimpleTimeBuffer<V> = StreamControlledBuffer<V, Box<source::TimerStream>>;
pub type SimpleTimeBufferredStream<S> = BufferedStream<S, SimpleTimeBuffer<<S as Stream>::Item>>;

pub fn new_simple_time_buffer<V>(time_span: u64) -> SimpleTimeBuffer<V> {
    StreamControlledBuffer {
        vec: Vec::<V>::new(),
        s: Box::pin(source::interval(time_span)),
    }
}

impl<S: Stream> SimpleTimeBufferredStream<S> {
    pub fn new(s: S, time_span: u64) -> Self {
        SimpleTimeBufferredStream {
            s: s.fuse(),
            buffer: StreamControlledBuffer {
                vec: vec![],
                s: Box::pin(source::interval(time_span))
            },
        }
    }
}


pub type SimpleExternalTimeBuffer<'a, V> = StreamControlledBuffer<V, &'a mut source::TimerStream>;
pub type SimpleExternalTimeBufferredStream<'a, S> = BufferedStream<S, SimpleExternalTimeBuffer<'a, <S as Stream>::Item>>;

impl<'a, S: Stream> SimpleExternalTimeBufferredStream<'a, S> {
    pub fn new_with_timer_stream(s: S, control_stream: Pin<&'a mut source::TimerStream>) -> Self {
        SimpleExternalTimeBufferredStream {
            s: s.fuse(),
            buffer: StreamControlledBuffer {
                vec: vec![],
                s: control_stream
            },
        }
    }
}

