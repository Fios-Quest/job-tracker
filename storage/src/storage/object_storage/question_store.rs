use crate::storable::Question;
use crate::storage::*;

pub trait QuestionStore:
    BaseStore<Question> + RecallById<Question> + RecallByName<Question> + RecallByRole<Question>
{
}
