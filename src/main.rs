use rxstream::source;
use tokio::prelude::*;
use rxstream::operators::RxStreamEx;

fn main() {
    let task = source::interval(1000).take(5).pairwise().for_each(|t| {
        println!("{:?}", t);
        Ok(())
    }).map_err(|e| panic!("err={:?}", e));
    
    tokio::run(task);
}
