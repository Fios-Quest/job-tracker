use crate::components::Editable;
use crate::router::DetailsView;
use crate::views::home::role_list::forms::edit_role_name::EditRoleName;
use crate::Route;
use application_context::prelude::*;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn RoleListItem(role: Arc<Role>, reload_roles: Callback) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let is_editable = use_signal::<bool>(|| false);

    let id = role.id;
    let company_id = role.id;
    let name = role.name.clone();

    let checked = context().get_role().map(|r| r.id) == Some(role.id);
    let display = rsx! {
        input {
            id: "{id}",
            r#type: "radio",
            name: "role",
            checked,
            onchange: move |_| {
                spawn(async move {
                    navigator()
                        .push(Route::HomeRole {
                            company_id,
                            role_id: id,
                            view: DetailsView::Role,
                        });
                });
            },
        }
        label { r#for: "{id}", "{name}" }
    };

    let callback = use_callback(move |_role| reload_roles(()));

    let editable = rsx! {
        EditRoleName { role, callback }
    };

    rsx! {
        li {
            Editable { display, editable, is_editable }
        }
    }
}
