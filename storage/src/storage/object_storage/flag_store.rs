use crate::prelude::RecallById;
use crate::storable::Flag;
use crate::storage::{BaseStore, RecallByCompany, RecallByName};

pub trait FlagStore:
    BaseStore<Flag> + RecallById<Flag> + RecallByName<Flag> + RecallByCompany<Flag>
{
}
