use futures::Stream;
mod join_creation;
mod transform;
pub use transform::pairwise::Pairwise;
pub use join_creation::combine_latest::CombineLatest;

pub use join_creation::combine_latest::combine_latest;

impl<T> RxStreamEx for T where T: Stream {}

pub trait RxStreamEx: Stream {
    fn pairwise(self) -> Pairwise<Self> 
        where Self::Item: Clone, Self: Sized 
    {
        Pairwise::new(self)
    }
    fn combine_latest<S>(self, other: S) -> CombineLatest<Self, S>
        where Self::Item: Clone, Self: Sized, S:Stream, S::Item: Clone 
    {
        CombineLatest::new(self, other)    
    }
}