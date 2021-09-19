use async_trait::async_trait;
use linearf::source::*;
use linearf::State;
use linearf::Vars;
use linearf::{Item, MaybeUtf8, New, Shared};
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

impl HasSourceParams for Rustdoc {
    type Params = ();
}

#[async_trait]
impl SimpleGenerator<()> for Rustdoc {
    async fn generate(&self, tx: Transmitter, _senario: (&Arc<Vars>, &Arc<Self::Params>)) {
        for i in 0..1000 {
            tx.chunk(
                (0..1000)
                    .map(|j| i + j + 1)
                    .map(|id| Item::new(id, "", MaybeUtf8::Utf8(i.to_string())))
                    .collect::<Vec<_>>(),
            )
        }
    }

    async fn reusable(
        &self,
        _prev: (&Arc<Vars>, &Arc<Self::Params>),
        _senario: (&Arc<Vars>, &Arc<Self::Params>),
    ) -> bool {
        false
    }
}
