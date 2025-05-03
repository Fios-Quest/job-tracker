use dioxus::prelude::*;
use storage::{ApplicationContext, Role};

#[component]
pub fn RoleListItem(role: Role) -> Element {
    let mut application_context = use_context::<Signal<ApplicationContext>>();
    let current_role = role.clone();
    let Role { id, name, .. } = role;

    rsx! {
        li { key: id,
            input {
                id: id.to_string(),
                r#type: "radio",
                name: "role",
                checked: false,
                onchange: move |_| {
                    let role = current_role.clone();
                    application_context
                        .set(application_context().set_role(role).expect("CompanyId not set"))
                },
            }
            label { r#for: id.to_string(), "{name}" }
        }
    }
}
