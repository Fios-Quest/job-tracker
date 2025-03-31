use dioxus::prelude::*;
use storage::{Flag, FlagColor};

#[component]
pub fn FlagListItem(flag: Flag) -> Element {
    let Flag {
        id,
        flag_color,
        name,
        ..
    } = flag;
    let flag_icon = match flag_color {
        FlagColor::Green => "💚",
        FlagColor::Red => "🚩",
    };

    let id = id.to_string();

    rsx! {
        li {
            id: "flag-{id}",
            "{flag_icon} {name}"
        }
    }
}
