pub use matchers::Substring;
pub use sources::Concat;

mod sources {
    pub struct Concat {}
    pub struct Files {}
}

mod matchers {
    use linearf::{
        async_trait,
        matcher::{BlankParams, *},
        session::Vars,
        Item, New, Shared, State,
    };
    use std::sync::Arc;

    pub struct Substring {
        _state: Shared<State>,
    }

    impl New for Substring {
        fn new(_state: &Shared<State>) -> Self
        where
            Self: Sized,
        {
            Self {
                _state: _state.clone(),
            }
        }
    }

    impl IsMatcher for Substring {
        type Params = BlankParams;
    }

    #[async_trait]
    impl SimpleScorer<BlankParams> for Substring {
        async fn score(
            &self,
            (vars, _): (&Arc<Vars>, &Arc<Self::Params>),
            item: &Arc<Item>,
        ) -> Score {
            return if item.view_for_matcing().find(&vars.query).is_some() {
                Score::new(item.id, vec![1])
            } else {
                Score::new(item.id, vec![])
            };
        }

        async fn reusable(
            &self,
            (prev, _): (&Arc<Vars>, &Arc<Self::Params>),
            (senario, _): (&Arc<Vars>, &Arc<Self::Params>),
        ) -> bool {
            prev.query == senario.query
        }
    }
}
