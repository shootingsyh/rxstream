use futures::{Stream, StreamExt, FutureExt, future};

pub fn fork_join<S1: Stream, S2: Stream>(s1: S1, s2: S2) -> 
    impl Stream<Item=(S1::Item, S2::Item)> 
{
    let f = future::join(
            s1.fold(None, |_, x| future::ready(Some(x))), 
            s2.fold(None, |_, x| future::ready(Some(x)))
        ).map(|x| match x {
            (Some(x1), Some(x2)) => Some((x1, x2)),
            _ => None
        }).into_stream().take_while(|x| future::ready(x.is_some())).map(|x| x.unwrap());
    f
}