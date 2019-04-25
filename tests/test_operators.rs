use rxstream::source;
use rxstream::operators::*;
use rxstream::operators::RxStreamEx;
use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;


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
fn merge_merge_two() {
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::interval(10).take(3);
    let t2 = source::timer(3, 10).take(4);
    let combined = merge(t1, t2).collect();
    let r = runtime.block_on(combined).unwrap();
    assert_eq!(r, vec![0, 0, 1, 1, 2, 2, 3])
}