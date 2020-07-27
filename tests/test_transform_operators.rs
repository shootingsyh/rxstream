use rxstream::source;
use rxstream::operators::RxStreamEx;
use futures::StreamExt;


#[tokio::test]
async fn pairwise_emit_pairs() {
    let f = source::of(0..).take(3).pairwise().collect::<Vec<_>>().await;
    assert_eq!(f, vec![(0,1),(1, 2)])
}

#[tokio::test]
async fn simple_count_buffer_emit_vecs() {
    let f = source::of(0..).buffer_count(3).take(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]])
}

#[tokio::test]
async fn simple_count_buffer_emit_partially_buffered_vecs() {
    let f = source::of(0..).take(8).buffer_count(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7]])
}

#[tokio::test]
async fn simple_count_buffer_emit_vecs_from_timer() {
    let f = source::interval(5).buffer_count(3).take(4).collect::<Vec<_>>().await;
    assert_eq!(f, vec![[0, 1, 2], [3, 4, 5], [6, 7, 8], [9, 10, 11]])
}

#[tokio::test]
async fn simple_time_buffer_emit_vecs_from_timer() {
    let f = source::interval(31).buffer_time(50).take(4).collect::<Vec<_>>().await;
    assert_eq!(f, vec![vec![0], vec![1, 2], vec![3], vec![4, 5]])
}

#[tokio::test]
async fn simple_time_buffer_emit_vecs_with_less_duration() {
    let f = source::interval_immediate(30).buffer_time(12).take(4).collect::<Vec<_>>().await;
    assert_eq!(f, vec![vec![0], vec![], vec![1], vec![]])
}

#[tokio::test]
async fn overlapped_count_buffer_skip_large_than_count() {
    let f = source::of(0..).buffer_count_with_skip(2, 3).take(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![[0, 1], [3, 4], [6, 7]])
}

#[tokio::test]
async fn overlapped_count_buffer_skip_large_than_count_with_leftover() {
    let f = source::of(0..).take(7).buffer_count_with_skip(2, 3).take(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![vec![0, 1], vec![3, 4], vec![6]])
}

#[tokio::test]
async fn overlapped_count_buffer_skip_smaller_than_count() {
    let f = source::of(0..).buffer_count_with_skip(3, 2).take(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![[0, 1, 2], [2, 3, 4], [4, 5, 6]])
}

#[tokio::test]
async fn overlapped_count_buffer_skip_smaller_than_count_with_leftover() {
    let f = source::of(0..).take(6).buffer_count_with_skip(3, 2).take(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![vec![0, 1, 2], vec![2, 3, 4], vec![4, 5]])
}

#[tokio::test]
async fn ovlapped_time_buffer_creation_time_large_than_span() {
    let f = source::interval(10)
        .buffer_time_with_creation_interval(
            35, 
            45
        ).take(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![[0, 1, 2], [5, 6, 7], [10, 11, 12]])
}

#[tokio::test]
async fn ovlapped_time_buffer_creation_time_smaller_than_span() {
    let f = source::interval(10)
        .buffer_time_with_creation_interval(
            35, /* time_span */
            15  /* creation_interval*/
        ).take(3).collect::<Vec<_>>().await;
    assert_eq!(f, vec![[0, 1, 2], [2, 3, 4], [4, 5, 6]])
}