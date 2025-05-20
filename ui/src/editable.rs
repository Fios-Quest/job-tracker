use dioxus::prelude::*;

#[component]
pub fn Editable(display: Element, edit_form: Element, form_id: String) -> Element {
    let mut is_editable = use_signal(|| false);

    rsx! {
        div { class: "editable",
            if is_editable() {
                {edit_form}
                " "
                button {
                    class: "button commit",
                    type: "submit",
                    form: form_id,
                    onclick: move |_| is_editable.set(false),
                    "✅"
                }
                a {
                    class: "button undo",
                    href: "#",
                    onclick: move |_| is_editable.set(false),
                    "❌"
                }
            } else {
                {display}
                " "
                a {
                    class: "button edit",
                    href: "#",
                    onclick: move |_| is_editable.set(true),
                    "🖋️"
                }
            }
        }
    }
}
