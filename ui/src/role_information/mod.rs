use dioxus::prelude::*;
use storage::{ApplicationContext, Role};

mod populated_role_description;
use crate::{Editable, StoreContext};
use populated_role_description::PopulatedRoleDescription;

#[component]
pub fn RoleDescription(role: Option<Role>) -> Element {
    let stores = use_context::<StoreContext>();
    let mut application_context = use_context::<Signal<ApplicationContext>>();

    let mut role_resource = use_resource(|| async {
        if let Some(role) = use_context::<Signal<ApplicationContext>>()().get_role() {
            use_context::<StoreContext>().get_role(role).await.ok()
        } else {
            None
        }
    });

    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    match role_resource().unwrap_or_default() {
        Some(role) => {
            let input_name = "role_description";

            if let Some(event) = form_receiver() {
                let stores = stores.clone();
                let role = role.clone();
                let role_description = event.values().get(input_name).map(|v| v.as_value());
                spawn(async move {
                    if let Some(description) = role_description {
                        if !description.is_empty() {
                            let role = Role {
                                description: description.clone(),
                                ..role
                            };
                            let _result = stores.update_role(&role).await;
                            let new_context = application_context().set_role(role.clone()).unwrap(); // ToDo: Fix me
                            application_context.set(new_context);
                            role_resource.restart();
                            form_receiver.set(None);
                        }
                    }
                });
            }

            let editable = rsx! {
                textarea {
                    name: input_name,
                    "{role.description}"
                }
            };
            let display = rsx! { PopulatedRoleDescription { role } };

            rsx! {
                h3 { "Role Description" }
                Editable { display, editable, form_receiver }
            }
        }
        None => rsx!(),
    }
}
