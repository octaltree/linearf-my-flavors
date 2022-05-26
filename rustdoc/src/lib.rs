pub use item_uri::A as ItemUri;
pub use rustdoc::S as Rustdoc;

mod rustdoc {
    use linearf::{item::*, source::*};
    use rayon::prelude::*;
    use std::path::PathBuf;
    use tokio_stream::wrappers::UnboundedReceiverStream;

    pub struct S<L> {
        linearf: Weak<L>
    }

    #[derive(Deserialize, Serialize, PartialEq)]
    pub struct P {
        dir: PathBuf
    }

    impl SourceParams for P {}

    impl<L> IsSource for S<L> {
        type Params = P;
    }

    impl<L> NewSource<L> for S<L>
    where
        L: linearf::Linearf + Send + Sync + 'static
    {
        fn new(linearf: Weak<L>) -> Source<<Self as IsSource>::Params>
        where
            Self: Sized
        {
            Source::from_simple(Self { linearf })
        }
    }

    impl<L> SimpleGenerator for S<L>
    where
        L: linearf::Linearf
    {
        fn stream(
            &self,
            (_vars, params): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync>> {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            let dir = Some(params.dir.clone());
            let lnf = match self.linearf.upgrade() {
                Some(l) => l,
                None => return Box::pin(futures::stream::empty())
            };
            let rt = lnf.runtime();
            rt.spawn(async move {
                use rustdoc_index::{
                    doc::Crate, read_search_index, search_index::search_indexes, Error
                };
                if let Ok(indexes) = search_indexes(dir).await {
                    for search_index in indexes.into_iter() {
                        let doc = match read_search_index(search_index) {
                            Err(_) => continue,
                            Ok(x) => x
                        };
                        let _ = doc.try_for_each(
                            |r: Result<(String, Crate), Error>| -> Result<(), Error> {
                                let (_name, krate) = r?;
                                for x in krate.items() {
                                    let _ = tx.send(MaybeUtf8::Utf8(x));
                                }
                                Ok(())
                            }
                        );
                    }
                }
            });
            Box::pin(
                UnboundedReceiverStream::new(rx)
                    .enumerate()
                    .map(|(i, v)| Item::new(u32::try_from(i).unwrap() + 1, v))
            )
        }

        fn reusable(
            &self,
            (_p, prev): (&Arc<Vars>, &Arc<Self::Params>),
            (_v, params): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Reusable {
            if prev == params {
                Reusable::Cache
            } else {
                Reusable::None
            }
        }
    }
}

mod item_uri {
    use linearf::action::*;
    use std::path::PathBuf;

    pub struct A<L> {
        linearf: Weak<L>
    }

    #[derive(Deserialize, Serialize, PartialEq)]
    pub struct P {
        dir: PathBuf,
        value: String
    }

    pub type R = Option<String>;

    impl ActionParams for P {}

    impl<L> IsAction for A<L> {
        type Params = P;
        type Result = R;
    }

    impl<L> NewAction<L> for A<L>
    where
        L: linearf::Linearf + Send + Sync + 'static
    {
        fn new(linearf: Weak<L>) -> Action<<Self as IsAction>::Params, <Self as IsAction>::Result>
        where
            Self: Sized
        {
            Action::from_simple(Self { linearf })
        }
    }

    impl<L> SimpleTask for A<L>
    where
        L: linearf::Linearf
    {
        fn run(&self, params: &Arc<<Self as IsAction>::Params>) -> <Self as IsAction>::Result {
            let lnf = self.linearf.upgrade()?;
            lnf.runtime()
                .block_on(async {
                    rustdoc_index::location::location_from_line(&params.value).await
                })
                .ok()
        }
    }
}
