use crate::value_list::forms::edit_value::EditValue;
use crate::Editable;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn ValueListItem(value: Arc<Value>, reload_values: Callback) -> Element {
    let mut is_editable = use_signal(|| false);
    let callback = use_callback(move |_value| {
        reload_values(());
        is_editable.set(false);
    });

    let display = rsx! {
        header { "{value.name}" }
        "{value.description}"
    };

    let editable = rsx! {
        EditValue { value: value.clone(), callback }
    };

    rsx! {
        li { id: "value-{value.id}",
            Editable { display, editable, is_editable }
        }
    }
}
