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
    async fn query(&mut self, query: &str, flow: &std::sync::Arc<Flow>) { todo!() }

    async fn score(&mut self, item: &Item) -> Score { todo!() }
}
