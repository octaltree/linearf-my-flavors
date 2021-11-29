mod clap {
    use filter::{
        matcher::{Bonus, FuzzyAlgorithm, MatchType, Query},
        SourceItem
    };
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
            let query: Query = Query::from(&*vars.query);
            let matcher =
                filter::matcher::Matcher::with_bonuses(FuzzyAlgorithm::Fzy, todo!(), todo!());
            let result =
                match matcher.match_query(&SourceItem::from(&*item.view_for_matcing()), &query) {
                    Some(x) => x,
                    None => return Score::new_excluded()
                };
            Score::value(item.id, result.0)
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
