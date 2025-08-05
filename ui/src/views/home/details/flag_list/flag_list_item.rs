use crate::helpers::{unwrap_or_report_and_return, wrap_in_thunk, ModifyWithFormData};
use crate::{Editable, StoreType};
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn FlagListItem(flag: Flag, reload_flags: Callback) -> Element {
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);
    let stores = use_context::<StoreType>();

    let id = flag.id;

    let input_name = "name";
    let input_color = "flag_color";

    let flag_icon = match flag.flag_color {
        FlagColor::Green => "💚",
        FlagColor::Red => "🚩",
    };

    let display = rsx! { "{flag_icon} {flag.name}" };
    let editable: Element = rsx! {
        select { name: input_color,
            option { selected: flag.flag_color == FlagColor::Red, value: "red", "🚩 Red" }
            option { selected: flag.flag_color == FlagColor::Green, value: "green", "💚 Green" }
        }
        input { name: input_name, value: "{flag.name}" }
    };

    if let Some(event) = form_receiver() {
        wrap_in_thunk! {
            let mut stores = stores.clone();
            let mut flag = flag;
            if flag.modify_with_form_data(&event).is_ok() && !flag.name.is_empty() {
                spawn(async move {
                    unwrap_or_report_and_return!(stores.store(flag).await);
                    reload_flags(());
                    form_receiver.set(None);
                });
            }
        }
    }

    rsx! {
        li { id: "flag-{id}",
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}
