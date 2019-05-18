use rxstream::source;
use rxstream::operators::*;
use rxstream::operators::RxStreamEx;
use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;
use std::time::{Duration};

#[test]
fn combine_latest_combines_two() {
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::interval_immediate(10).take(3);
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

#[test]
fn test_concat_vec_concats_all() {
    let t1 = source::of(0..).take(3);
    let t2 = source::of(1..).take(3);
    let t3 = source::of(2..).take(3);
    let r = concat_vec(vec![t1, t2, t3]).collect().wait().unwrap();
    assert_eq!(r, [0,1,2,1,2,3,2,3,4])
}

#[test]
fn test_concat_all_concats_all() {
    let mut runtime = Runtime::new().unwrap();
    let t = concat_all(
        source::of(0..3).map(|i| source::timer(i*3, 10).take(3))
    ).collect();
    let r = runtime.block_on(t).unwrap();
    assert_eq!(r, [0,1,2,0,1,2,0,1,2])
}

#[test]
fn test_race_pick_first_respond_item() {
    fn m2(i: u64) -> u64 {
        return i * 2;
    }
    fn m21(i: u64) -> u64 {
        return i * 2 + 1;
    }
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::timer(3, 10).map(m2 as fn(u64) -> u64).take(3);
    let t2 = source::timer(1, 10).map(m21 as fn(u64) -> u64).take(6);
    let raced = race(t1, t2).collect();
    let r = runtime.block_on(raced).unwrap();
    assert_eq!(r, [1,3,5,7,9,11])
}

#[test]
fn test_race_pick_first_ended() {
    fn m2(i: u64) -> u64 {
        return i * 2;
    }
    fn m21(i: u64) -> u64 {
        return i * 2 + 1;
    }
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::timer(3, 10).map(m2 as fn(u64) -> u64).take(3);
    let t2 = source::timer(1, 10).map(m21 as fn(u64) -> u64).take(0);
    let raced = race(t1, t2).collect();
    let r = runtime.block_on(raced).unwrap();
    assert_eq!(r, [])
}

#[test]
fn test_with_latest_from_sync_the_stream() {
    let mut runtime = Runtime::new().unwrap();
    let s1 = source::interval(5).take(3);
    let s2 = source::interval(1).take(100);
    let merged = s1.with_latest_from(s2).collect();
    let r = runtime.block_on(merged).unwrap();
    assert_eq!(r, [(0, 4), (1, 9), (2, 14)])
}
