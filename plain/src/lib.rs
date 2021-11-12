pub use identity::S as Identity;
pub use substring::M as Substring;

mod identity {
    use linearf::{item::*, source::*};

    pub struct S<L> {
        _linearf: Weak<L>
    }

    #[derive(Deserialize, Serialize, PartialEq, Clone)]
    pub enum Str {
        Utf8(String),
        Bytes(Vec<u8>)
    }

    #[derive(Deserialize, Serialize, PartialEq)]
    pub struct P {
        values: Vec<Str>
    }

    impl SourceParams for P {}

    impl<L> IsSource for S<L> {
        type Params = P;
    }

    impl<L> NewSource<L> for S<L>
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

    impl<L> SimpleGenerator for S<L> {
        fn stream(
            &self,
            (_, params): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Pin<Box<dyn Stream<Item = Item> + Send + Sync>> {
            let it = params.values.clone().into_iter().enumerate();
            let s = futures::stream::unfold(it, |mut it| async {
                it.next().map(|(i, x)| {
                    let i: u32 = i.try_into().unwrap();
                    let id = i + 1;
                    let item = Item::new(
                        id,
                        match x {
                            Str::Utf8(s) => MaybeUtf8::Utf8(s),
                            Str::Bytes(b) => MaybeUtf8::Bytes(b)
                        }
                    );
                    (item, it)
                })
            });
            Box::pin(s)
        }

        fn reusable(
            &self,
            (_, prev): (&Arc<Vars>, &Arc<Self::Params>),
            (_, params): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Reusable {
            if prev == params {
                Reusable::Same
            } else {
                Reusable::None
            }
        }
    }
}

mod substring {
    use linearf::matcher::*;

    pub struct M<L> {
        _linearf: Weak<L>
    }

    impl<L> IsMatcher for M<L> {
        type Params = BlankParams;
    }

    impl<L> NewMatcher<L> for M<L>
    where
        L: linearf::Linearf + Send + Sync + 'static
    {
        fn new(_linearf: Weak<L>) -> Matcher<<Self as IsMatcher>::Params>
        where
            Self: Sized
        {
            Matcher::from_simple(Self { _linearf })
        }
    }

    impl<L> SimpleScorer for M<L> {
        fn score(&self, (vars, _): (&Arc<Vars>, &Arc<Self::Params>), item: &Arc<Item>) -> Score {
            return if item.view_for_matcing().find(&vars.query).is_some() {
                Score::new(item.id, vec![1])
            } else {
                Score::new(item.id, vec![])
            };
        }

        fn reusable(
            &self,
            (prev, _): (&Arc<Vars>, &Arc<Self::Params>),
            (senario, _): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Reusable {
            if prev.query == senario.query {
                Reusable::Same
            } else {
                Reusable::None
            }
        }
    }
}
