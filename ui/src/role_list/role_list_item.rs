use crate::editable::Editable;
use crate::{Route, StoreType};
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn RoleListItem(role: Role, reload_roles: Callback) -> Element {
    let stores = use_context::<StoreType>();
    let context = use_context::<Signal<ApplicationContext>>();
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let Role {
        id,
        name,
        company_id,
        ..
    } = role.clone();

    let checked = context().get_role().map(|r| r.id) == Some(role.id);
    let display = rsx! {
        input {
            id: id.to_string(),
            r#type: "radio",
            name: "role",
            checked,
            onchange: move |_| {
                spawn(async move {
                    navigator()
                        .push(Route::HomeRole {
                            company_id,
                            role_id: id,
                        });
                });
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
