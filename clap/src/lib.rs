pub use clap::M as Clap;

mod clap {
    use filter::{
        matcher::{Bonus, FuzzyAlgorithm, MatchingTextKind, Query},
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
            if vars.query.is_empty() {
                return Score::value(item.id, 1);
            }
            let query: Query = Query::from(&*vars.query);
            let bonuses = vec![Bonus::FileName];
            let ty = MatchingTextKind::Full;
            let matcher = filter::matcher::Matcher::with_bonuses(bonuses, FuzzyAlgorithm::Fzy, ty);
            let result =
                match matcher.match_query(&SourceItem::from(&*item.view_for_matcing()), &query) {
                    Some(x) => x,
                    None => return Score::new_excluded()
                };
            Score::value(item.id, -result.score)
        }

        fn reusable(
            &self,
            (prev, _): (&Arc<Vars>, &Arc<Self::Params>),
            (scenario, _): (&Arc<Vars>, &Arc<Self::Params>)
        ) -> Reusable {
            if prev.query == scenario.query {
                Reusable::Same
            } else {
                Reusable::None
            }
        }
    }
}
