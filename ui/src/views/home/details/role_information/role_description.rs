use crate::components::Editable;
use crate::helpers::unwrap_or_report_and_return;
use crate::prelude::Route;
use crate::role_information::forms::EditRoleDescription;
use crate::role_information::populated_role_description::PopulatedRoleDescription;
use crate::router::DetailsView;
use application_context::prelude::ApplicationContext;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::Role;

#[component]
pub fn RoleDescription(role: Arc<Role>) -> Element {
    let is_editable = use_signal(|| false);

    let callback = use_callback(move |role: Role| {
        let role_id = role.id;
        let company_id = role.company_id;
        let mut context = use_context::<Signal<ApplicationContext>>();
        let new_context = unwrap_or_report_and_return!(context().set_role(role));
        context.set(new_context);
        navigator().push(Route::HomeRole {
            company_id,
            role_id,
            view: DetailsView::Role,
        });
    });

    let editable = rsx! {
        EditRoleDescription { role: role.clone(), callback }
    };
    let display = rsx! {
        PopulatedRoleDescription { role }
    };

    rsx! {
        h3 { "Role Description" }

        Editable { display, editable, is_editable }
    }
}
