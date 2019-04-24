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
    timer(0, millis)
}

pub fn of<T: Iterator>(iter: T) -> impl Stream<Item = T::Item, Error = ()> {
    futures::stream::iter_ok(iter)
}