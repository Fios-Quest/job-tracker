use crate::flag_list::forms::edit_flag::EditFlag;
use crate::Editable;
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn FlagListItem(flag: Flag, reload_flags: Callback) -> Element {
    let is_editable = use_signal::<bool>(|| false);

    let id = flag.id;

    let flag_icon = match flag.flag_color {
        FlagColor::Green => "💚",
        FlagColor::Red => "🚩",
    };

    let display = rsx! { "{flag_icon} {flag.name}" };
    let editable: Element = rsx! {
        EditFlag { flag, callback: reload_flags }
    };

    rsx! {
        li { id: "flag-{id}",
            Editable { display, editable, is_editable }
        }
    }
}
