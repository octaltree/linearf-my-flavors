use linearf::{matcher::Matcher, *};

pub struct Clap {}

impl New for Clap {
    fn new(_state: &Shared<State>, _rt: &AsyncRt) -> Self
    where
        Self: Sized
    {
        Clap {}
    }
}

impl Matcher for Clap {
    async fn score(&mut self, flow: &Arc<Flow>, query: &Query, item: &Item) -> Score { todo!() }
}
