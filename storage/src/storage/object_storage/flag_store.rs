use crate::storable::Flag;
use crate::storage::*;

pub trait FlagStore:
    BaseStore<Flag> + RecallById<Flag> + RecallByName<Flag> + RecallByCompany<Flag>
{
}
