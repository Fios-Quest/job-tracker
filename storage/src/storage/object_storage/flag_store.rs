use crate::storable::Flag;
use crate::storage::{BaseStore, RecallByCompany, RecallByName};

pub trait FlagStore: BaseStore<Flag> + RecallByName<Flag> + RecallByCompany<Flag> {}
