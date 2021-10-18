use linearf::{item::*, source::*};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct Rustdoc<L> {
    _linearf: Weak<L>
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct RustdocParams {
    dir: PathBuf
}

impl SourceParams for RustdocParams {}

impl<L> IsSource for Rustdoc<L> {
    type Params = RustdocParams;
}

impl<L> NewSource<L> for Rustdoc<L>
where
    L: linearf::Linearf + Send + Sync + 'static
{
    fn new(_linearf: Weak<L>) -> Source<<Self as IsSource>::Params>
    where
        Self: Sized
    {
        Source::from_simple(Self { _linearf })
    }
}

impl<L> SimpleGenerator for Rustdoc<L> {
    fn stream(
        &self,
        (_, params): (&Arc<Vars>, &Arc<Self::Params>)
    ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync>> {
        todo!()
        // let s = futures::stream::unfold(0..1000000, |mut it| async {
        //    it.next().map(|i| {
        //        let id = i + 1;
        //        let item = Item::new(id, "number", MaybeUtf8::Utf8(i.to_string()));
        //        (item, it)
        //    })
        //});
        // Box::pin(s)
    }

    fn reusable(
        &self,
        (_, prev): (&Arc<Vars>, &Arc<Self::Params>),
        (_, params): (&Arc<Vars>, &Arc<Self::Params>)
    ) -> Reusable {
        if prev == params {
            Reusable::Cache
        } else {
            Reusable::None
        }
    }
}
