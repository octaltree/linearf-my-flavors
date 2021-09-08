use async_trait::async_trait;
use linearf::{
    source::{Generator, Transmitter},
    AsyncRt, Flow, Item, MaybeUtf8, New, Session, Shared, State
};
use std::sync::Arc;

pub struct Rustdoc {
    _state: Shared<State>,
    _rt: AsyncRt
}

impl New for Rustdoc {
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
impl Generator for Rustdoc {
    async fn generate(
        &mut self,
        tx: Transmitter,
        flow: &Arc<Flow>
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // for i in 0..1000000 {
        //    tx.item(Item {
        //        idx: i + 1,
        //        value: linearf::StringBytes::String(i.to_string()),
        //        r#type: "",
        //        view: None,
        //        view_for_matcing: None,
        //        query: None
        //    })?;
        //}
        for i in 0..1000 {
            tx.chunk(
                (0..1000)
                    .map(|j| i + j + 1)
                    .map(|id| Item::new(id, "", MaybeUtf8::Utf8(i.to_string())))
                    .collect::<Vec<_>>()
            )?;
        }
        Ok(())
    }

    async fn reusable(&self, _prev: &Session, _flow: &Arc<Flow>) -> bool { false }
}
