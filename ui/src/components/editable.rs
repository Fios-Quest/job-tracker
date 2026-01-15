use dioxus::prelude::*;

#[component]
fn EditableEdit(editable: Element, is_editable: Signal<bool>) -> Element {
    rsx! {
        div { class: "editable edit", {editable} }
    }
}

#[component]
fn EditableDisplay(display: Element, is_editable: Signal<bool>) -> Element {
    rsx! {
        div { class: "editable display",
            {display}
            " "
            button {
                class: "edit",
                r#type: "button",
                onclick: move |_| is_editable.set(true),
                "🖋️"
            }
        }
    }
}

#[component]
pub fn Editable(display: Element, editable: Element, is_editable: Signal<bool>) -> Element {
    match is_editable() {
        true => rsx! {
            EditableEdit { editable, is_editable }
        },
        false => rsx! {
            EditableDisplay { display, is_editable }
        },
    }
}
