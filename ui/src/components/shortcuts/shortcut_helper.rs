use crate::{ShortcutEvent, SHORTCUT_SIGNAL};
use dioxus::prelude::*;

#[component]
pub fn ShortcutLink(shortcut_event: ShortcutEvent, to: String, children: Element) -> Element {
    if Some(shortcut_event) == SHORTCUT_SIGNAL() {
        *SHORTCUT_SIGNAL.write() = None;
        navigator().push(to.clone());
    }
    rsx! {
        div { class: "shortcut-helper",
            Link { to, {children} }
            div { class: "helper-text", "{shortcut_event}" }
        }
    }
}
