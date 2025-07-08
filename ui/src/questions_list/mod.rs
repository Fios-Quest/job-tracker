mod populated_question_list;

use dioxus::events::FormData;
pub use populated_question_list::*;
use std::sync::Arc;
use storage::prelude::*;

mod question_list_item;
pub use question_list_item::*;

const QUESTION_NAME_FIELD: &str = "question_name";
const QUESTION_ANSWER_FIELD: &str = "question_answer";

fn form_data_name_and_answer(form_data: &FormData) -> Option<(String, String)> {
    let name = form_data.values().get(QUESTION_NAME_FIELD)?.as_value();
    let answer = form_data
        .values()
        .get(QUESTION_ANSWER_FIELD)
        .map(|v| v.as_value())
        .unwrap_or_default();
    Some((name, answer))
}

fn create_question_from_form_data(role: Arc<Role>, form_data: &FormData) -> Option<Question> {
    let (name, answer) = form_data_name_and_answer(form_data)?;
    if name.is_empty() {
        None
    } else {
        Some(role.create_question(name, answer))
    }
}

fn edit_question_from_form_data(question: Arc<Question>, form_data: &FormData) -> Option<Question> {
    let (name, answer) = form_data_name_and_answer(form_data)?;
    if name.is_empty() {
        None
    } else {
        Some(Question::new(question.role_id, name, answer))
    }
}
