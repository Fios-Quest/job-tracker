use dioxus::prelude::*;

#[component]
fn EditableEdit(
    editable: Element,
    form_receiver: Signal<Option<Event<FormData>>>,
    is_editable: Signal<bool>,
) -> Element {
    rsx! {
        div { class: "editable edit",
            form {
                onsubmit: move |e| {
                    form_receiver.set(Some(e));
                    is_editable.set(false);
                },
                {editable}
                " "
                button { class: "button commit", r#type: "submit", "✅" }
                button {
                    class: "button undo",
                    r#type: "button",
                    onclick: move |_| is_editable.set(false),
                    "❌"
                }
            }
        }
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
pub fn Editable(
    display: Element,
    editable: Element,
    form_receiver: Signal<Option<Event<FormData>>>,
) -> Element {
    let is_editable = use_signal(|| false);

    match is_editable() {
        true => rsx! {
            EditableEdit { editable, form_receiver, is_editable }
        },
        false => rsx! {
            EditableDisplay { display, is_editable }
        },
    }
}
