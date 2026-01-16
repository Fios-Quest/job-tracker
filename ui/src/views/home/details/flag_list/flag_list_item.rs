use crate::flag_list::forms::edit_flag::EditFlag;
use crate::Editable;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn FlagListItem(flag: Arc<Flag>, reload_flags: Callback) -> Element {
    let is_editable = use_signal(|| false);

    let id = flag.id;

    let flag_icon = match flag.flag_color {
        FlagColor::Green => "💚",
        FlagColor::Red => "🚩",
    };

    let callback = use_callback(move |_flag| reload_flags(()));

    let display = rsx! { "{flag_icon} {flag.name}" };
    let editable: Element = rsx! {
        EditFlag { flag, callback }
    };

    rsx! {
        li { id: "flag-{id}",
            Editable { display, editable, is_editable }
        }
    }
}
