use crate::EMIT_ERROR;
use dioxus::prelude::*;

// ToDo: Figure out how to listen for an event a single time
// const ERROR_SHORTCUT: ShortcutEvent = ShortcutEvent {
//     shortcut_modifier: ShortcutModifier::Ctrl,
//     shortcut_key: ShortcutKey::E,
// };

#[component]
pub fn ErrorDisplay() -> Element {
    let mut show_errors = use_signal(|| false);
    let mut errors = use_signal(Vec::<String>::new);

    use_effect(move || {
        if let Some(error) = EMIT_ERROR() {
            show_errors.set(true);
            errors.with_mut(move |e| e.push(error));
        }
    });

    // use_memo(move || {
    //     if let Some(ERROR_SHORTCUT) = SHORTCUT_SIGNAL() {
    //         show_errors.toggle();
    //     }
    // });

    let show_class = if show_errors() { "show-errors" } else { "" };

    let error_strings = errors();
    let error_list = error_strings.iter().map(|e| {
        rsx! {
            li { "{e}" }
        }
    });

    rsx! {
        div { class: "error-container {show_class}",
            div { class: "show-errors-button",
                a {
                    href: "#",
                    onclick: move |e| {
                        show_errors.toggle();
                        e.prevent_default();
                    },
                    if show_errors() {
                        "X"
                    } else {
                        "<"
                    }
                }
            }
            div { class: "error-display",
                ul { {error_list} }
            }
        }
    }
}
