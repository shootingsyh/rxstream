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
fn simple_count_buffer_emit_vecs() {
    let f = source::of(0..).buffer(3).take(3).collect().wait().unwrap();
    assert_eq!(f, vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]])
}


#[test]
fn simple_count_buffer_emit_vecs_from_timer() {
    let mut runtime = Runtime::new().unwrap();
    let f = source::interval(5).buffer(3).take(4).collect();
    let r = runtime.block_on(f).unwrap();
    assert_eq!(r, vec![[0, 1, 2], [3, 4, 5], [6, 7, 8], [9, 10, 11]])
}