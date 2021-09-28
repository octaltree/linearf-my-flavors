use linearf::{
    async_trait,
    source::{BlankParams, *},
    Item, MaybeUtf8, New, Shared, State, Vars,
};
use std::sync::Arc;

pub struct Rustdoc {
    _state: Shared<State>,
}

impl New for Rustdoc {
    fn new(_state: &Shared<State>) -> Self
    where
        Self: Sized,
    {
        Self {
            _state: _state.clone(),
        }
    }
}

impl IsSource for Rustdoc {
    type Params = BlankParams;
}

#[async_trait]
impl SimpleGenerator<BlankParams> for Rustdoc {
    async fn generate(&self, tx: Transmitter, _senario: (&Arc<Vars>, &Arc<Self::Params>)) {
        for i in 0..1000 {
            tx.chunk(
                (0..1000)
                    .map(|j| i * 1000 + j + 1)
                    .map(|id| Item::new(id, "", MaybeUtf8::Utf8(i.to_string())))
                    .collect::<Vec<_>>(),
            )
            .await;
        }
        //for i in 0..1000000 {
        //    let tx = tx.clone();
        //    tokio::spawn(async move {
        //        tx.item(Item::new(i + 1, "", MaybeUtf8::Utf8(i.to_string())))
        //            .await;
        //    });
        //}
    }

    async fn reusable(
        &self,
        _prev: (&Arc<Vars>, &Arc<Self::Params>),
        _senario: (&Arc<Vars>, &Arc<Self::Params>),
    ) -> bool {
        false
    }
}
