use futures::Stream;
use futures::stream::Select;
mod join_creation;
mod transform;
pub use transform::pairwise::Pairwise;
pub use join_creation::combine_latest::CombineLatest;

// static operators
pub use join_creation::combine_latest::combine_latest;
pub fn merge<S1: Stream, S2: Stream<Item=S1::Item, Error=S1::Error>>(s1: S1, s2: S2) -> Select<S1, S2> {
    s1.select(s2)
}

impl<T> RxStreamEx for T where T: Stream {}

// function operators
pub trait RxStreamEx: Stream {
    fn pairwise(self) -> Pairwise<Self> 
        where Self::Item: Clone, Self: Sized 
    {
        Pairwise::new(self)
    }
}