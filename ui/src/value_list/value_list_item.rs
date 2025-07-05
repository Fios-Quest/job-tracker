use super::{form_date_to_value, VALUE_DESCRIPTION_FIELD, VALUE_NAME_FIELD};
use crate::{Editable, StoreType};
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn ValueListItem(value: Value, reload_values: Callback) -> Element {
    let value_id = value.id;
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    if let Some(event) = form_receiver() {
        let value = form_date_to_value(value.company_id, &event);
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
        li { id: "flag-{value.id}",
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}
