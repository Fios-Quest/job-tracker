use dioxus::prelude::*;
use storage::{ApplicationContext, Role};

#[component]
pub fn RoleListItem(role: Role) -> Element {
    let mut application_context = use_context::<Signal<ApplicationContext>>();
    let Role { id, name, .. } = role;

    rsx! {
        li {
            key: id,
            input {
                id: id.to_string(),
                r#type: "radio",
                name: "role",
                onchange: move |_| application_context.set(application_context().set_role_id(id).unwrap()),
            }
            label {
                for: id.to_string(),
                "{name}"
            }
        }
    }
}
