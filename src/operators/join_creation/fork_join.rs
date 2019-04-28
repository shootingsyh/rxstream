extern crate either;
use either::{Either};
use futures::{Stream, Async, Poll, Future, future};

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct ForkJoinFromJoinedFuturesIntoStream<F>(Option<F>);

pub fn fork_join<S1: Stream, S2: Stream>(s1: S1, s2: S2) -> 
    impl Stream<Item=(S1::Item, S2::Item), Error=Either<S1::Error, S2::Error>> 
{
    ForkJoinFromJoinedFuturesIntoStream(
        Some(s1.fold(None, |_, x| future::ok(Some(x))).map_err(|e| Either::Left(e)).join(
            s2.fold(None, |_, x| future::ok(Some(x))).map_err(|e| Either::Right(e))
        ))
    )
}

impl<I1, I2, E1, E2, F> Stream for ForkJoinFromJoinedFuturesIntoStream<F>
    where F: Future<Item=(Option<I1>, Option<I2>), Error=Either<E1, E2>> 
{
    type Item = (I1, I2);
    type Error = Either<E1, E2>;
    
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let ret = match self.0 {
            None => return Ok(Async::Ready(None)),
            Some(ref mut f) => {
                match f.poll() {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Err(e) => Err(e),
                    Ok(Async::Ready((Some(r1), Some(r2)))) => Ok(Async::Ready(Some((r1, r2)))),
                    _ => Ok(Async::Ready(None)),
                }
            }
        };
        self.0 = None;
        ret
    }
}