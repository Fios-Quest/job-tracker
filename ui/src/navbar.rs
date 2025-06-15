use dioxus::prelude::*;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        div { class: "flex gap-4 my-4", {children} }
    }
}
