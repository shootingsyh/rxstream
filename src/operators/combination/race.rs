use futures::task::Poll;
use futures::task::Context;
use std::pin::Pin;
use futures::Stream;
use pin_project::pin_project;

#[pin_project(project=RaceProj)]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Race<S: Stream> {
    #[pin]
    s1: S,
    #[pin]
    s2: S,
    state: RaceState,
}

#[derive(Debug)]
enum RaceState {
    Pending,
    Pick1,
    Pick2,
}

impl<S: Stream> Race<S> {
    pub fn new(stream1: S, stream2: S) -> Race<S>
    {
        Race{s1: stream1, s2: stream2, state: RaceState::Pending}
    }
}

pub fn race<S: Stream>(s1: S, s2: S) -> Race<S> {
    Race::new(s1, s2)
}

impl<S: Stream> Stream for Race<S> {
    type Item = S::Item;

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.state {
            RaceState::Pending => {
                match this.s1.poll_next(cx) {
                    Poll::Pending => (),
                    x => {
                        *this.state = RaceState::Pick1;
                        return x
                    }
                };
                match this.s2.poll_next(cx) {
                    Poll::Pending => Poll::Pending,
                    x => {
                        *this.state = RaceState::Pick2;
                        return x
                    }
                }
            },
            RaceState::Pick1 => return this.s1.poll_next(cx),
            RaceState::Pick2 => return this.s2.poll_next(cx),
        }
    }
}