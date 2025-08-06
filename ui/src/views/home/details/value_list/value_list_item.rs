use super::{VALUE_DESCRIPTION_FIELD, VALUE_NAME_FIELD};
use crate::helpers::{iife, unwrap_or_report_and_return, ModifyWithFormData};
use crate::{Editable, StoreType};
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn ValueListItem(value: Arc<Value>, reload_values: Callback) -> Element {
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    if let Some(event) = form_receiver() {
        iife! {
            let mut value = Arc::unwrap_or_clone(value.clone());
            unwrap_or_report_and_return!(value.modify_with_form_data(&event));
            let mut store = use_context::<StoreType>();
            spawn(async move {
                unwrap_or_report_and_return!(store.store(value).await);
                reload_values(());
                form_receiver.set(None);
            });
        }
    };

    let display = rsx! {
        header { "{value.name}" }
        "{value.description}"
    };

    let editable = rsx! {
        input { name: VALUE_NAME_FIELD, value: "{value.name}" }
        textarea { name: VALUE_DESCRIPTION_FIELD, value: "{value.description}" }
        input { r#type: "submit" }
    };

    rsx! {
        li { id: "value-{value.id}",
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}
