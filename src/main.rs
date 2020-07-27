use rxstream::source;
use futures::StreamExt;
use rxstream::operators::RxStreamEx;

#[tokio::main]
async fn main() -> () {
    source::interval(1000).take(5).pairwise().map(|t| {
        println!("{:?}", t)
    }).collect::<()>().await;
}