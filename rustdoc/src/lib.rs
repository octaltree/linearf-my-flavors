use async_trait::async_trait;
use linearf::{
    source::{Generator, Sender},
    AsyncRt, Flow, Item, New, Session, Shared, State
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Rustdoc {}

impl New for Rustdoc {
    fn new(_state: &Shared<State>, _rt: &AsyncRt) -> Self
    where
        Self: Sized
    {
        Self {}
    }
}

#[async_trait]
impl Generator for Rustdoc {
    async fn generate(
        &mut self,
        tx: Sender<Item>,
        flow: &Arc<Flow>
    ) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..1000000 {
            tx.send(Item {
                idx: i + 1,
                value: linearf::StringBytes::String(i.to_string()),
                r#type: "",
                view: None,
                view_for_matcing: None,
                query: None
            })?;
        }
        Ok(())
    }

    async fn reusable(&self, _prev: &Session, _flow: &Arc<Flow>) -> bool { false }
}
