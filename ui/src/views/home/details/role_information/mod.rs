use application_context::prelude::ApplicationContext;
use dioxus::prelude::*;
use std::sync::Arc;

mod populated_role_description;
use crate::helpers::{iife, unwrap_or_report_and_return, ModifyWithFormData};
use crate::router::DetailsView;
use crate::{Editable, Route, StoreType};
use populated_role_description::PopulatedRoleDescription;
use storage::prelude::*;

#[component]
pub fn RoleDescription(role: Arc<Role>) -> Element {
    let stores = use_context::<StoreType>();
    let mut context = use_context::<Signal<ApplicationContext>>();
    let input_name = "description";

    let editable = rsx! {
        textarea { name: input_name, "{role.description}" }
    };
    let display = rsx! {
        PopulatedRoleDescription { role: role.clone() }
    };

    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);
    if let Some(event) = form_receiver() {
        iife! {
            let mut stores = stores.clone();
            let mut role = Arc::unwrap_or_clone(role);
            unwrap_or_report_and_return!(role.modify_with_form_data(&event));
            let role_id = role.id;
            let company_id = role.company_id;
            spawn(async move {
                unwrap_or_report_and_return!(stores.store(role.clone()).await);
                let new_context = unwrap_or_report_and_return!(context().set_role(role));
                context.set(new_context);
                navigator().push(Route::HomeRole {
                    company_id,
                    role_id,
                    view: DetailsView::Role,
                });
                form_receiver.set(None);
            });
        }
    }

    rsx! {
        h3 { "Role Description" }
        if form_receiver().is_none() {
            Editable { display, editable, form_receiver }
        } else {
            "pending"
        }
    }
}
