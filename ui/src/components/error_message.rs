use dioxus::prelude::*;

#[component]
pub fn ErrorMessage(message: String) -> Element {
    rsx! {
        div { class: "error-message", {message} }
    }
}
