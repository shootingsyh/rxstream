use futures::stream::{Stream, StreamExt};
use tokio::time;
use std::time::{Duration};

pub type TimerStream = impl StreamExt<Item = u64>;

pub fn timer(initial: u64, period: u64) -> TimerStream {
    let iter = 0u64..;
    let b = time::interval_at(
        time::Instant::now() + Duration::from_millis(initial), 
        Duration::from_millis(period),
    ).zip(futures::stream::iter(iter)).map(|r| r.1);
    b
}


pub fn interval(millis: u64) -> TimerStream {
    timer(millis, millis)
}

/// Interval which emit the first value immediately rather than wait for the first period pass
pub fn interval_immediate(millis: u64) -> TimerStream {
    timer(0, millis)
}

/// This is for both of and range in rxjs
pub fn of<T: IntoIterator>(iter: T) -> impl Stream<Item = T::Item> {
    futures::stream::iter(iter)
}

// /// create a stream which emit error immediately
// pub fn throw_error<E, S: Stream<Error=E>>(error: E) -> futures::stream::Once<S::Item, E> {
//     futures::stream::once::<S::Item, E>(Err(error))
// } 

pub use futures::stream::empty;