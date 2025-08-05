use super::role_list_item::RoleListItem;
use crate::helpers::{unwrap_or_report_and_return, CreatePartialFromFormData};
use crate::router::DetailsView;
use crate::Route::HomeRole;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn PopulatedRoleList(company: Arc<Company>) -> Element {
    let stores = use_context::<StoreType>();
    let role_name_value = use_signal(|| "");

    // Get roles for the company
    let mut roles_resource = use_resource(use_reactive!(|(company)| async move {
        unwrap_or_report_and_return!(
            use_context::<StoreType>()
                .recall_by_company(company.id)
                .await
        )
    }));
    let reload_roles = use_callback(move |()| roles_resource.restart());
    let roles = roles_resource().unwrap_or_default();
    let roles_list = roles.into_iter().map(|role| {
        rsx! {
            RoleListItem { role, reload_roles }
        }
    });

    let create_role = move |event: Event<FormData>| {
        let mut stores = stores.clone();
        let company = company.clone();
        async move {
            let mut partial_role =
                unwrap_or_report_and_return!(PartialRole::from_form_data(&event));
            partial_role.date_applied = Some(Timestamp::now());

            let role = unwrap_or_report_and_return!(company.create_role_from_partial(partial_role));

            let company_id = company.id;
            let role_id = role.id;

            // Store the name
            unwrap_or_report_and_return!(stores.store(role).await);

            // Navigate away from the page
            navigator().push(HomeRole {
                company_id,
                role_id,
                view: DetailsView::Role,
            });
        }
    };

    rsx! {
        div { id: "roles",

            h3 { "Roles" }

            ul { {roles_list} }

            form { class: "flex flex-col", onsubmit: create_role,
                input { id: "add_role", name: "name", value: role_name_value }
                input { r#type: "submit" }
            }
        }
    }
}
