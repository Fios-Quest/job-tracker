use crate::editable::Editable;
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::*;
use storage::ApplicationContext;

#[component]
pub fn RoleListItem(role: Role, reload_roles: Callback) -> Element {
    let mut application_context = use_context::<Signal<ApplicationContext>>();
    let stores = use_context::<StoreType>();
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let Role { id, name, .. } = role.clone();

    let current_role = role.clone();
    let display = rsx! {
        input {
            id: id.to_string(),
            r#type: "radio",
            name: "role",
            checked: false,
            onchange: move |_| {
                application_context
                    .set(
                        application_context()
                            .set_role(current_role.clone())
                            .expect("CompanyId not set"),
                    )
            },
        }
        label { r#for: id.to_string(), "{name}" }
    };

    if let Some(event) = form_receiver() {
        let mut stores = stores.clone();
        let role_name = event.values().get("role_name").map(|v| v.as_value());
        spawn(async move {
            if let Some(name) = role_name {
                if !name.is_empty() {
                    let role = Role {
                        name,
                        ..role.clone()
                    };
                    let _result = stores // ToDo: Handle errors
                        .store(role)
                        .await;
                    reload_roles(());
                    form_receiver.set(None);
                }
            }
        });
    }

    let editable = rsx! {
        input {
            id: id.to_string(),
            r#type: "text",
            name: "role_name",
            value: name,
        }
    };

    rsx! {
        li { key: id,
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}
