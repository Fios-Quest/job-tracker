use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::{ApplyPartial, BaseStore, PartialQuestion, Question, QuestionFieldName};
use uuid::Uuid;

fn create_on_submit(question: Question, callback: Callback<Uuid>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<PartialQuestion>().map_err(log_error) {
            let mut question = question.clone();
            spawn(async move {
                let id = question.id;
                question.apply(form_data);
                let mut stores = use_context::<StoreType>();
                report_if_error!(stores.store(question).await);
                callback(id);
            });
        }
    }
}

#[component]
pub fn EditQuestion(question: Question, callback: Callback<Uuid>) -> Element {
    let name = question.name.clone();
    let answer = question.answer.clone();
    rsx! {
        form { onsubmit: create_on_submit(question, callback),
            input { name: QuestionFieldName::Name.name(), value: name }
            textarea { name: QuestionFieldName::Answer.name(), value: answer }
            input { r#type: "submit" }
        }
    }
}
