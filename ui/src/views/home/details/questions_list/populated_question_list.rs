use crate::helpers::unwrap_or_report_and_return;
use crate::questions_list::forms::CreateQuestion;
use crate::questions_list::QuestionListItem;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn QuestionList(role: Arc<Role>) -> Element {
    let role_id = role.id;

    let mut questions_resource = use_resource(use_reactive!(|(role_id)| async move {
        let questions: Vec<Question> =
            unwrap_or_report_and_return!(use_context::<StoreType>().recall_by_role(role_id).await);
        questions
    }));

    let reload_questions = use_callback(move |()| questions_resource.restart());
    let questions = questions_resource().unwrap_or_default();
    let questions_list = questions.into_iter().map(Arc::new).map(move |question| {
        rsx! {
            QuestionListItem { question, reload_questions }
        }
    });

    let callback = use_callback(move |_question| questions_resource.restart());

    rsx! {
        div { id: "flags",
            h3 { "Questions" }
            ul { {questions_list} }

            CreateQuestion { role, callback }
        }
    }
}
