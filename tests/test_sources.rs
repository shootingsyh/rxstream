use rxstream::source;
use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;
mod common;

#[test]
fn of_generates_list() {
    let f = source::of(0..).take(5).collect();
    let r = f.wait();
    assert_eq!(r, Ok(vec![0,1,2,3,4]))
}
#[test]
fn timer_generates_list() {
    let mut runtime = Runtime::new().unwrap();
    // need tokio run time for timer
    let t = source::interval(10)
        .take(3)
        .collect();
    let r = runtime.block_on(t).unwrap();
    assert_eq!(r, vec![0,1,2])
}

#[test]
fn timer_generates_timed_list() {
    //generating two timed list, and see their order
    //need tokio run time for timer
    let mut runtime = Runtime::new().unwrap();
    let t1 = source::interval(10).take(3);
    let t2 = source::timer(3, 10).take(6);
    let t = t1.select(t2).collect();
    let r = runtime.block_on(t).unwrap();
    assert_eq!(r, vec![0, 0, 1, 1, 2, 2, 3, 4, 5])
}
