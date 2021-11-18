pub use identity::Id as Identity;
pub use line::C as FormatLine;
pub use substring::M as Substring;

mod identity {
    use linearf::{converter::*, item::*, matcher::*, source::*};

    pub struct Id<L> {
        _linearf: Weak<L>
    }

    #[derive(Deserialize, Serialize, PartialEq, Clone)]
    #[serde(untagged)]
    pub enum Str {
        Utf8(String),
        Bytes(Vec<u8>)
    }

    #[derive(Deserialize, Serialize, PartialEq)]
    pub struct P {
        values: Vec<Str>
    }

    impl SourceParams for P {}

    impl<L> IsSource for Id<L> {
        type Params = P;
    }

    impl<L> NewSource<L> for Id<L>
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

    impl<L> SimpleGenerator for Id<L> {
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

    impl<L> IsMatcher for Id<L> {
        type Params = BlankParams;
    }

    impl<L> NewMatcher<L> for Id<L>
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

    impl<L> SimpleScorer for Id<L> {
        fn score(&self, (_vars, _): (&Arc<Vars>, &Arc<Self::Params>), item: &Arc<Item>) -> Score {
            return Score::new(item.id, vec![1]);
        }

        fn reusable(
            &self,
            (_prev, _): (&Arc<Vars>, &Arc<Self::Params>),
            (_senario, _): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Reusable {
            Reusable::Same
        }
    }

    impl<L> IsConverter for Id<L> {
        type Params = Void;
    }

    impl<L> NewConverter<L> for Id<L>
    where
        L: linearf::Linearf + Send + Sync + 'static
    {
        fn new(_linearf: Weak<L>) -> Converter<<Self as IsConverter>::Params>
        where
            Self: Sized
        {
            Converter::from_simple(Self { _linearf })
        }
    }

    impl<L> SimpleConverter for Id<L> {
        fn convert(&self, item: Item) -> Item { item }
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

    fn parse_query(q: &str) -> Vec<&str> {
        // TODO: support double quoted string
        q.split_whitespace().collect()
    }

    impl<L> SimpleScorer for M<L> {
        fn score(&self, (vars, _): (&Arc<Vars>, &Arc<Self::Params>), item: &Arc<Item>) -> Score {
            let q = parse_query(&vars.query);
            return if q
                .into_iter()
                .all(|q| item.view_for_matcing().find(q).is_some())
            {
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
            let mut p = parse_query(&prev.query);
            let mut v = parse_query(&senario.query);
            p.sort_unstable();
            v.sort_unstable();
            if p == v {
                Reusable::Same
            } else {
                Reusable::None
            }
        }
    }
}

mod line {
    use linearf::{converter::*, item::MaybeUtf8};
    use std::marker::PhantomData;

    pub struct C<L> {
        phantom: PhantomData<L>
    }

    impl<L> IsConverter for C<L> {
        type Params = Void;
    }

    impl<L> NewConverter<L> for C<L>
    where
        L: linearf::Linearf + Send + Sync + 'static
    {
        fn new(_linearf: Weak<L>) -> Converter<<Self as IsConverter>::Params>
        where
            Self: Sized
        {
            Converter::from_simple(Self {
                phantom: PhantomData
            })
        }
    }

    impl<L> SimpleConverter for C<L> {
        fn convert(&self, mut item: Item) -> Item {
            item.view_for_matcing = Some(item.value_lossy().to_string());
            item.view = Some(format!("{}:{}", item.id, item.value_lossy())); // TODO: padding
            item.value = MaybeUtf8::Utf8(item.id.to_string());
            item
        }
    }
}
