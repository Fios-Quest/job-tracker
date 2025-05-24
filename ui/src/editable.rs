use dioxus::prelude::*;

#[component]
pub fn Editable(
    display: Element,
    editable: Element,
    form_receiver: Signal<Option<Event<FormData>>>,
) -> Element {
    let mut is_editable = use_signal(|| false);

    rsx! {
        div { class: "editable",
            if is_editable() {
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
            } else {
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
}
