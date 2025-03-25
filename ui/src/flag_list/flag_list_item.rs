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

    rsx! {
        li {
            key: id.to_string(),
            "{flag_icon} {name}"
        }
    }
}
