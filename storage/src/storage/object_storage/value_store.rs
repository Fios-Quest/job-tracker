use crate::prelude::Value;
use crate::storage::*;

pub trait ValueStore:
    BaseStore<Value> + RecallById<Value> + RecallByName<Value> + RecallByCompany<Value>
{
}
