use super::{edit_value_from_form_data, VALUE_DESCRIPTION_FIELD, VALUE_NAME_FIELD};
use crate::{Editable, StoreType};
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn ValueListItem(value: Arc<Value>, reload_values: Callback) -> Element {
    let value_id = value.id;
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    if let Some(event) = form_receiver() {
        let value = edit_value_from_form_data(value.clone(), &event);
        if let Some(mut value) = value {
            value.id = value_id;
            spawn(async move {
                let mut store = use_context::<StoreType>();
                let _result = store.store(value).await;
                reload_values(());
                form_receiver.set(None);
            });
        } else {
            form_receiver.set(None);
        }
    }

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
