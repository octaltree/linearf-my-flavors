//    use crate::{Flow, Item, Session, State};
//    use std::{stream::Stream, sync::Arc};
//    use tokio::sync::{mpsc, RwLock};

//    #[derive(Debug)]
//    pub struct StateSource {
//        state: Arc<RwLock<State>>
//    }

//    #[derive(Debug)]
//    pub struct StateSession {
//        state: Arc<RwLock<State>>
//    }

//    #[derive(Debug)]
//    pub struct StateFlow {
//        state: Arc<RwLock<State>>
//    }
