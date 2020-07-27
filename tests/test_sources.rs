use rxstream::source;
use futures::StreamExt;
use futures::stream::select;
mod common;

#[tokio::test]
async fn of_generates_list() {
    let f = source::of(0..).take(5).collect::<Vec<_>>();
    let r = f.await;
    assert_eq!(r, vec![0,1,2,3,4])
}
#[tokio::test]
async fn timer_generates_list() {
    let r = source::interval(10)
        .take(3)
        .collect::<Vec<_>>().await;
    assert_eq!(r, vec![0,1,2])
}

#[tokio::test]
async fn timer_generates_timed_list() {
    //generating two timed list, and see their order
    let t1 = source::interval(10).take(3);
    let t2 = source::timer(3, 10).take(6);
    let r = select(t1, t2).collect::<Vec<_>>().await;
    assert_eq!(r, vec![0, 0, 1, 1, 2, 2, 3, 4, 5])
}
