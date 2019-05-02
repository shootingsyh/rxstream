use core::mem;
use futures::{Stream, Async, Poll};


#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Race<S: Stream>(RaceState<S>);

#[derive(Debug)]
enum RaceState<S: Stream> {
    Pending(S, S),
    Pick(S),
    Temp,
}

impl<S: Stream> Race<S> {
    pub fn new(stream1: S, stream2: S) -> Race<S>
    {
        Race(RaceState::Pending(stream1, stream2))
    }
}

pub fn race<S: Stream>(s1: S, s2: S) -> Race<S> {
    Race::new(s1, s2)
}

impl<S: Stream> Stream for Race<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match &mut self.0 {
            RaceState::Pending(s1, s2) => {
                match s1.poll() {
                    Ok(Async::NotReady) => (),
                    x => {
                        self.0 = match mem::replace(&mut self.0, RaceState::Temp) {
                            RaceState::Pending(s1, _s2) => RaceState::Pick(s1),
                            _ => unreachable!(),
                        };
                        return x
                    }
                };
                match s2.poll() {
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    x => {
                        self.0 = match mem::replace(&mut self.0, RaceState::Temp) {
                            RaceState::Pending(_s1, s2) => RaceState::Pick(s2),
                            _ => unreachable!(),
                        };
                        return x
                    }
                }
            },
            RaceState::Pick(s) => return s.poll(),
            RaceState::Temp => unreachable!(),
        }
    }
}