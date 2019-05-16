use futures::Stream;
extern crate either;
use either::{Either, Left, Right};
use futures::stream::Select;
use futures::Future;
mod combination;
mod transform;
pub use transform::pairwise::Pairwise;
pub use combination::combine_latest::CombineLatest;
pub use combination::combine_latest::CombineLatestVec;
pub use combination::with_latest_from::WithLatestFrom;
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
    impl Stream<Item=Vec<SInner::Item>, Error=Either<SInner::Error, SOuter::Error>> 
    where SInner::Item: Clone
{
    s.collect()
        .map(|streams| combine_latest_vec(streams).map_err(|e| Left(e)))
        .map_err(|e| Right(e))
        .flatten_stream()
}

/// merge is an alias of select operator in rust stream library. 
/// Notes 
/// 1. merge in rust stream library is a deprecated operator, and replaced by select. 
/// 2. The 'concurrent' parameter is not supported due to we only support two operands.
pub fn merge<S1: Stream, S2: Stream<Item=S1::Item, Error=S1::Error>>(s1: S1, s2: S2) -> Select<S1, S2> {
    s1.select(s2)
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

pub fn start_with<S: Stream, V: IntoIterator<Item=S::Item>>(v: V, s: S) -> impl Stream<Item=S::Item, Error=S::Error> {
    concat(source::of_with_err_type(v), s)
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
}