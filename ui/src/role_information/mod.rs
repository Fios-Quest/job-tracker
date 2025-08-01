use dioxus::prelude::*;
use std::sync::Arc;

mod populated_role_description;
use crate::router::DetailsView;
use crate::{Editable, Route, StoreType};
use application_context::prelude::*;
use populated_role_description::PopulatedRoleDescription;
use storage::prelude::*;

#[component]
pub fn RoleDescription(role: Arc<Role>) -> Element {
    let stores = use_context::<StoreType>();
    let mut context = use_context::<Signal<ApplicationContext>>();

    let input_name = "role_description";

    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);
    if let Some(event) = form_receiver() {
        let mut stores = stores.clone();
        let role_description = event.values().get(input_name).map(|v| v.as_value());
        let role = role.clone();
        spawn(async move {
            if let Some(description) = role_description {
                if !description.is_empty() {
                    let edited_role = Role {
                        description,
                        ..role.as_ref().clone()
                    };
                    let _result = stores.store(edited_role.clone()).await;
                    let nav = navigator();
                    nav.push(Route::HomeRole {
                        company_id: edited_role.company_id,
                        role_id: edited_role.id,
                        view: DetailsView::Role,
                    });
                    form_receiver.set(None);
                    // ToDo: Why isn't this updating the role in context?! 😡
                    context.set(context().set_role(edited_role).expect("Couldn't set role"));
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
