use super::role_list_item::RoleListItem;
use crate::helpers::unwrap_or_report_and_return;
use crate::router::DetailsView;
use crate::views::home::role_list::forms::create_role::CreateRole;
use crate::Route::HomeRole;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn PopulatedRoleList(company: Arc<Company>) -> Element {
    let company_id = company.id;

    // Get roles for the company
    let mut roles_resource = use_resource(use_reactive!(|(company_id)| async move {
        unwrap_or_report_and_return!(
            use_context::<StoreType>()
                .recall_by_company(company_id)
                .await
        )
    }));
    let reload_roles = use_callback(move |()| roles_resource.restart());
    let roles = roles_resource().unwrap_or_default();
    let roles_list = roles.into_iter().map(Arc::new).map(|role| {
        rsx! {
            RoleListItem { role, reload_roles }
        }
    });

    let on_create_role = use_callback(move |role: Role| {
        // Navigate away from the page
        navigator().push(HomeRole {
            company_id,
            role_id: role.id,
            view: DetailsView::Role,
        });
    });

    rsx! {
        div { id: "roles",

            h3 { "Roles" }

            ul { {roles_list} }

            CreateRole { company, callback: on_create_role }
        }
    }
}
