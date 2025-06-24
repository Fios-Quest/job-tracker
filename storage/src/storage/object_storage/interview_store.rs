use crate::storable::Interview;
use crate::storage::*;

pub trait InterviewStore:
    BaseStore<Interview> + RecallById<Interview> + RecallByName<Interview> + RecallByRole<Interview>
{
}
