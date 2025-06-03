use crate::storable::object::flag::Flag;
use crate::storage::property::base_store::BaseStore;
use crate::storage::property::recall_by_company::RecallByCompany;
use crate::storage::property::recall_by_name::RecallByName;

pub trait FlagStore: BaseStore<Flag> + RecallByName<Flag> + RecallByCompany<Flag> {}
