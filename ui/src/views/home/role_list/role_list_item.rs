use crate::components::Editable;
use crate::helpers::{unwrap_or_report_and_return, ModifyWithFormData};
use crate::router::DetailsView;
use crate::{Route, StoreType};
use application_context::prelude::*;
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn RoleListItem(role: Role, reload_roles: Callback) -> Element {
    let stores = use_context::<StoreType>();
    let context = use_context::<Signal<ApplicationContext>>();
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);

    let Role {
        id,
        name,
        company_id,
        ..
    } = role.clone();

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
        label { r#for: id.to_string(), "{name}" }
    };

    if let Some(event) = form_receiver() {
        let mut stores = stores.clone();
        let mut role = role;
        spawn(async move {
            unwrap_or_report_and_return!(role.modify_with_form_data(&event));
            unwrap_or_report_and_return!(
                stores // ToDo: Handle errors
                    .store(role)
                    .await
            );
            reload_roles(());
            form_receiver.set(None);
        });
    }

    let editable = rsx! {
        input {
            id: id.to_string(),
            r#type: "text",
            name: "name",
            value: name,
        }
    };

    rsx! {
        li {
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}
