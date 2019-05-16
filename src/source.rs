use futures::Stream;
use tokio::timer::Interval;
use std::time::{Duration, Instant};

pub fn timer(initial: u64, period: u64) -> impl Stream<Item = u64, Error = tokio::timer::Error> {
    let iter = 0u64..;
    Interval::new(
        Instant::now() + Duration::from_millis(initial), 
        Duration::from_millis(period),
    ).zip(futures::stream::iter_ok(iter)).map(|r| r.1)
}


pub fn interval(millis: u64) -> impl Stream<Item = u64, Error = tokio::timer::Error> {
    timer(millis, millis)
}

/// Interval which emit the first value immediately rather than wait for the first period pass
pub fn interval_immediate(millis: u64) -> impl Stream<Item = u64, Error = tokio::timer::Error> {
    timer(0, millis)
}

/// This is for both of and range in rxjs
pub fn of<T: IntoIterator>(iter: T) -> impl Stream<Item = T::Item, Error = ()> {
    futures::stream::iter_ok(iter)
}

/// A version of 'of' to allow specify an error type, though it won't throw error.
pub fn of_with_err_type<T: IntoIterator, E>(iter: T) -> impl Stream<Item = T::Item, Error = E> {
    futures::stream::iter_ok(iter)
}

/// create a stream which emit error immediately
pub fn throw_error<E, S: Stream<Error=E>>(error: E) -> futures::stream::Once<S::Item, E> {
    futures::stream::once::<S::Item, E>(Err(error))
} 

pub use futures::stream::empty;