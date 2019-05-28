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
    let f = source::of(0..).buffer_count(3).take(3).collect().wait().unwrap();
    assert_eq!(f, vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]])
}

#[test]
fn simple_count_buffer_emit_partially_buffered_vecs() {
    let f = source::of(0..).take(8).buffer_count(3).collect().wait().unwrap();
    assert_eq!(f, vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7]])
}

#[test]
fn simple_count_buffer_emit_vecs_from_timer() {
    let mut runtime = Runtime::new().unwrap();
    let f = source::interval(5).buffer_count(3).take(4).collect();
    let r = runtime.block_on(f).unwrap();
    assert_eq!(r, vec![[0, 1, 2], [3, 4, 5], [6, 7, 8], [9, 10, 11]])
}

#[test]
fn simple_time_buffer_emit_vecs_from() {
    let mut runtime = Runtime::new().unwrap();
    let f = source::interval(30).buffer_time(Duration::from_millis(50)).take(4).collect();
    let r = runtime.block_on(f).unwrap();
    assert_eq!(r, vec![vec![0], vec![1, 2], vec![3, 4], vec![5, 6]])
}

#[test]
fn overlapped_count_buffer_skip_large_than_count() {
    let f = source::of(0..).buffer_count_with_skip(2, 3).take(3).collect().wait().unwrap();
    assert_eq!(f, vec![[0, 1], [3, 4], [6, 7]])
}

#[test]
fn overlapped_count_buffer_skip_large_than_count_with_leftover() {
    let f = source::of(0..).take(7).buffer_count_with_skip(2, 3).take(3).collect().wait().unwrap();
    assert_eq!(f, vec![vec![0, 1], vec![3, 4], vec![6]])
}

#[test]
fn overlapped_count_buffer_skip_smaller_than_count() {
    let f = source::of(0..).buffer_count_with_skip(3, 2).take(3).collect().wait().unwrap();
    assert_eq!(f, vec![[0, 1, 2], [2, 3, 4], [4, 5, 6]])
}

#[test]
fn overlapped_count_buffer_skip_smaller_than_count_with_leftover() {
    let f = source::of(0..).take(6).buffer_count_with_skip(3, 2).take(3).collect().wait().unwrap();
    assert_eq!(f, vec![vec![0, 1, 2], vec![2, 3, 4], vec![4, 5]])
}