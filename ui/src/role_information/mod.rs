use dioxus::prelude::*;
use uuid::Uuid;

mod populated_role_description;
use crate::{Editable, Route, StoreType};
use populated_role_description::PopulatedRoleDescription;
use storage::prelude::*;

#[component]
pub fn RoleDescription(role_id: Option<Uuid>) -> Element {
    let stores = use_context::<StoreType>();

    let role_resource: Resource<Option<Role>> =
        use_resource(use_reactive!(|(role_id)| async move {
            if let Some(role_id) = role_id {
                let temp: Option<Role> =
                    use_context::<StoreType>().recall_by_id(&role_id).await.ok();
                temp
            } else {
                None
            }
        }));

    let form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let Some(role): Option<Role> = role_resource().unwrap_or_default() else {
        return rsx! {};
    };

    let input_name = "role_description";

    if let Some(event) = form_receiver() {
        let mut stores = stores.clone();
        let role = role.clone();
        let role_description = event.values().get(input_name).map(|v| v.as_value());
        spawn(async move {
            if let Some(description) = role_description {
                if !description.is_empty() {
                    let role = Role {
                        description,
                        ..role.clone()
                    };
                    let _result = stores.store(role.clone()).await;
                    let nav = navigator();
                    nav.push(Route::HomeRole {
                        company_id: role.company_id,
                        role_id: role.id,
                    });
                }
            }
        });
    }

    let editable = rsx! {
        textarea { name: input_name, "{role.description}" }
    };
    let display = rsx! {
        PopulatedRoleDescription { role }
    };

    rsx! {
        h3 { "Role Description" }
        if form_receiver().is_none() {
            Editable { display, editable, form_receiver }
        } else {
            "pending"
        }
    }
}
