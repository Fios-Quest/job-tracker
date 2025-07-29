use dioxus::prelude::*;

#[component]
pub fn ShortcutHelper(helper_text: String, children: Element) -> Element {
    rsx! {
        div { class: "shortcut-helper",
            {children}
            div { class: "helper-text", {helper_text} }
        }
    }
}
