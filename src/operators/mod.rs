use futures::Stream;
extern crate either;
pub use futures::stream;
pub use futures::stream::{Select, Chain, Once};
pub use futures::future;
pub use futures::future::Join;
mod join_creation;
mod transform;
pub use transform::pairwise::Pairwise;
pub use join_creation::combine_latest::CombineLatest;

// static operators
pub use join_creation::combine_latest::combine_latest;

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
pub fn concat<S1: Stream, S2: Stream<Item=S1::Item, Error=S1::Error>>(s1: S1, s2: S2) -> Chain<S1, S2> {
    s1.chain(s2)
}

/// run both stream to the end, and yield the tuple of both stream's last value as value then end. 
/// Notes
/// 1. The error type is either of the error happened. 
/// 2. If any of the stream end without value, the result stream will be empty (end without value)
pub use join_creation::fork_join::fork_join;

impl<T> RxStreamEx for T where T: Stream {}

// function operators
pub trait RxStreamEx: Stream {
    fn pairwise(self) -> Pairwise<Self> 
        where Self::Item: Clone, Self: Sized 
    {
        Pairwise::new(self)
    }
}