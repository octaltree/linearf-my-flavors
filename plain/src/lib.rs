use async_trait::async_trait;
use linearf::{matcher::Matcher, AsyncRt, Flow, Item, New, Score, Shared, State};
use std::sync::Arc;

pub struct Concat {}

pub struct Substring {
    _state: Shared<State>,
    _rt: AsyncRt
}

impl New for Substring {
    fn new(_state: &Shared<State>, _rt: &AsyncRt) -> Self
    where
        Self: Sized
    {
        Self {
            _state: _state.clone(),
            _rt: _rt.clone()
        }
    }
}

#[async_trait]
impl Matcher for Substring {
    async fn score(&mut self, flow: &Arc<Flow>, query: &str, item: &Item) -> Score {
        // TODO
        Score::new(item.id, vec![0])
    }
}

// impl Generator for Concat
// dynamic concat??
// impl Matcher for Substring
