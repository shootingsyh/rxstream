use futures::Stream;
use tokio::timer::Interval;

pub fn interval(millis: u64) -> impl Stream<Item = u64, Error = tokio::timer::Error> {
    let iter = 0u64..;
    Interval::new_interval(std::time::Duration::from_millis(millis))
        .zip(futures::stream::iter_ok(iter))
        .map(|r| r.1)
}

pub fn of<T: Iterator>(iter: T) -> impl Stream<Item = T::Item, Error = ()> {
    futures::stream::iter_ok(iter)
}