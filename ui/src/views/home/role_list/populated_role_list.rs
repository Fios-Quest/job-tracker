use super::role_list_item::RoleListItem;
use crate::helpers::CreatePartialFromFormData;
use crate::router::DetailsView;
use crate::Route::HomeRole;
use crate::StoreType;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

fn handle_storage_error(error: anyhow::Error) -> Option<String> {
    tracing::error!("Storage Error: {:?}", error);

    match error.downcast_ref::<StorageError>() {
        Some(StorageError::NotFound) => Some("No role found".to_string()),
        Some(StorageError::AlreadyExists) => Some("Role already exists".to_string()),
        _ => Some("A database error has occurred".to_string()),
    }
}

#[component]
pub fn PopulatedRoleList(company: Arc<Company>) -> Element {
    let stores = use_context::<StoreType>();
    let role_name_value = use_signal(|| "");
    let mut error_message = use_signal(|| None);

    // Get roles for the company
    let mut roles_resource = use_resource(use_reactive!(|(company)| async move {
        let result = use_context::<StoreType>()
            .recall_by_company(company.id)
            .await;
        match result {
            Ok(roles) => roles,
            Err(e) => {
                error_message.set(handle_storage_error(e));
                Vec::with_capacity(0)
            }
        }
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
        let mut partial_role = PartialRole::from_form_data(&event)
            .expect("Could not create partial role from form data");
        partial_role.date_applied = Some(Timestamp::now());
        let role = company
            .create_role_from_partial(partial_role)
            .expect("Could not create role from partial");

        let company_id = company.id;
        let role_id = role.id;

        async move {
            // Store the name
            stores.store(role).await.expect("Could not store company");

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
