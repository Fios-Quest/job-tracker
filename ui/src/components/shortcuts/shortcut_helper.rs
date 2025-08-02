use crate::{ShortcutEvent, SHORTCUT_SIGNAL};
use dioxus::prelude::*;

#[component]
pub fn ShortcutHelper(
    shortcut_event: ShortcutEvent,
    on_shortcut: Callback,
    children: Element,
) -> Element {
    if Some(shortcut_event) == SHORTCUT_SIGNAL() {
        on_shortcut(());
    }
    rsx! {
        div { class: "shortcut-helper",
            {children}
            div { class: "helper-text", "{shortcut_event}" }
        }
    }
}
