use std::pin::Pin;
use futures::{Stream, StreamExt, FutureExt};
use futures::stream::Select;
mod combination;
mod transform;
pub use transform::pairwise::Pairwise;
pub use combination::combine_latest::CombineLatest;
pub use combination::combine_latest::CombineLatestVec;
pub use combination::with_latest_from::WithLatestFrom;
pub use transform::simple_count_buffer::SimpleCountBufferedStream;
pub use transform::overlapped_count_buffer::OverlappedCountBufferedStream;
pub use transform::simple_time_buffer::{SimpleExternalTimeBufferredStream, SimpleTimeBufferredStream};
pub use transform::overlapped_time_buffer::OverlappedTimeBufferedStream;
use super::source;

// static operators

/// combine latest
/// Notes:
/// 1. If any inner stream is done without emit any value, the result stream 
///    will end immediately. 
/// 2. As long as all inner stream emitted some value, the result stream
///    will end when all inner streams end
/// 3. If any inner streams didn't emit any value and didn't end, the result
///    stream wait forever.
pub use combination::combine_latest::combine_latest;
pub use combination::combine_latest::combine_latest_vec;

/// combine_all
/// See warning in combine_latest as this one use same logic there. 
pub fn combine_all<SInner: Stream, SOuter: Stream<Item=SInner>>(s: SOuter) -> 
    impl Stream<Item=Vec<SInner::Item>> 
    where SInner::Item: Clone
{
    s.collect()
        .map(|streams| combine_latest_vec(streams))
        .flatten_stream()
}

/// merge is an alias of select operator in rust stream library. 
/// Notes 
/// 1. merge in rust stream library is a deprecated operator, and replaced by select. 
/// 2. The 'concurrent' parameter is not supported due to we only support two operands.
pub fn merge<S1: Stream, S2: Stream<Item=S1::Item>>(s1: S1, s2: S2) -> Select<S1, S2> {
    futures::stream::select(s1, s2)
}

/// concat is an alias of chain operator in rust. 
/// Notes 
/// 1. concat in rust stream library means a totally different thing. Do not confuse with
/// the concat here which follows the rxjs naming convension. 
pub use combination::concat::concat;
pub use combination::concat::concat_vec;
pub use combination::concat::concat_all;

/// run both stream to the end, and yield the tuple of both stream's last value as value then end. 
/// Notes
/// 1. The error type is either of the error happened. 
/// 2. If any of the stream end without value, the result stream will be empty (end without value)
pub use combination::fork_join::fork_join;

/// merge two streams
pub use futures::stream::Zip;

/// Pick the first stream respond. 
pub use combination::race::race;

pub fn start_with<S: Stream, V: IntoIterator<Item=S::Item>>(v: V, s: S) -> impl Stream<Item=S::Item> {
    concat(source::of(v), s)
}

impl<T> RxStreamEx for T where T: Stream {}

// function operators
pub trait RxStreamEx: Stream {
    fn pairwise(self) -> Pairwise<Self> 
        where Self::Item: Clone, Self: Sized 
    {
        Pairwise::new(self)
    }

    fn with_latest_from<S2: Stream>(self, other: S2) -> WithLatestFrom<Self, S2>
        where S2::Item: Clone, Self: Sized
    {
        WithLatestFrom::new(self, other)
    }   

    fn buffer_count(self, count: usize) -> SimpleCountBufferedStream<Self> 
        where Self: Sized
    {
        SimpleCountBufferedStream::new(self, count)
    }

    fn buffer_count_with_skip(self, count: usize, skip: usize) -> OverlappedCountBufferedStream<Self>
        where Self: Sized, Self::Item: Clone
    {
        OverlappedCountBufferedStream::new(self, count, skip)
    }

    fn buffer_time(self, time_span: u64) -> SimpleTimeBufferredStream<Self> 
        where Self: Sized
    {
        SimpleTimeBufferredStream::new(self, time_span)
    }

    fn buffer_time_with_external_timer(self, timer_stream: Pin<&mut source::TimerStream>) -> SimpleExternalTimeBufferredStream<Self> 
        where Self: Sized
    {
        SimpleExternalTimeBufferredStream::new_with_timer_stream(self, timer_stream)
    }

    fn buffer_time_with_creation_interval(
        self, 
        time_span: u64, 
        creation_interval: u64,
    ) -> OverlappedTimeBufferedStream<Self> 
        where Self: Sized, Self::Item: Clone
    {
        OverlappedTimeBufferedStream::new(self, time_span, creation_interval)
    }

}