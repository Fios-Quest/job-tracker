use dioxus::prelude::*;
use storage::ApplicationContext;

mod populated_role_description;
use crate::{Editable, StoreType};
use populated_role_description::PopulatedRoleDescription;
use storage::prelude::*;

#[component]
pub fn RoleDescription(role: Option<Role>) -> Element {
    let stores = use_context::<StoreType>();
    let mut application_context = use_context::<Signal<ApplicationContext>>();

    let mut role_resource = use_resource(|| async {
        if let Some(role) = use_context::<Signal<ApplicationContext>>()().get_role() {
            use_context::<StoreType>().recall_by_id(role).await.ok()
        } else {
            None
        }
    });

    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

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
                    let new_context = application_context().set_role(role.clone()).unwrap(); // ToDo: Fix me
                    application_context.set(new_context);
                    role_resource.restart();
                    form_receiver.set(None);
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
