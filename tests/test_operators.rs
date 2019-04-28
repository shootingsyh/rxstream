use rxstream::source;
use rxstream::operators::*;
use rxstream::operators::RxStreamEx;
use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;
use std::time::{Duration};


#[test]
fn pairwise_emit_pairs() {
    let f = source::of(0..).take(3).pairwise().collect().wait().unwrap();
    assert_eq!(f, vec![(0,1),(1, 2)])
}

#[test]
fn combine_latest_combines_two() {
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::interval(10).take(3);
    let t2 = source::timer(3, 10).take(4);
    let combined = combine_latest(t1, t2).collect();
    let r = runtime.block_on(combined).unwrap();
    assert_eq!(r, vec![(0, 0), (1, 0), (1, 1), (2, 1), (2, 2), (2, 3)])
}

#[test]
fn combine_latest_end_soon_with_empty() {
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::empty::<i32, ()>();
    let t2 = source::timer(3, 10);
    let combined = combine_latest(t1, t2).collect().timeout(Duration::from_secs(1));
    let r = runtime.block_on(combined).unwrap();
    assert_eq!(r, vec![])
}

#[test]
fn combine_all_combines_all_streams_from_stream() {
    let mut runtime = Runtime::new().unwrap();
    let t = combine_all(
        source::of(0..3).map(|i| source::timer(i*3, 10).take(3))
    ).collect();
    let r = runtime.block_on(t).unwrap();
    assert_eq!(r, [[0,0,0], [1,0,0], [1,1,0], [1,1,1], [2,1,1], [2,2,1], [2,2,2]])
}

#[test]
fn merge_merge_two() {
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::interval(10).take(3);
    let t2 = source::timer(3, 10).take(4);
    let combined = merge(t1, t2).collect();
    let r = runtime.block_on(combined).unwrap();
    assert_eq!(r, vec![0, 0, 1, 1, 2, 2, 3])
}

#[test]
fn fork_join_join_two_iter_end() {
    let t1 = source::of(1..3);
    let t2 = source::of(3..).take(4);
    let r = fork_join(t1, t2).collect().wait().unwrap();
    assert_eq!(r, vec![(2, 6)])
}


#[test]
fn fork_join_join_two_interval_end() {
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::interval(10).take(3);
    let t2 = source::interval(10).take(6);
    let joined = fork_join(t1, t2).collect();
    let r = runtime.block_on(joined).unwrap();
    assert_eq!(r, vec![(2, 5)])
}

#[test]
fn fork_join_empty_when_any_empty() {
    let t1 = source::empty::<(), ()>();
    let t2 = source::of(3..).take(4);
    let r = fork_join(t1, t2).collect().wait().unwrap();
    assert_eq!(r, vec![])
}