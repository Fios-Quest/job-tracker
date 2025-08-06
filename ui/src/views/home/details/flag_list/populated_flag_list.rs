use super::flag_list_item::FlagListItem;
use crate::helpers::{unwrap_or_report_and_return, CreatePartialFromFormData};
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn PopulatedFlagList(company: Arc<Company>) -> Element {
    let company_id = company.id;

    let stores = use_context::<StoreType>();

    // Get flags for company
    let mut flags_resource = use_resource(use_reactive!(|(company_id)| async move {
        unwrap_or_report_and_return!(
            use_context::<StoreType>()
                .recall_by_company(company_id)
                .await
        )
    }));
    let reload_flags = use_callback(move |()| flags_resource.restart());
    let flags = flags_resource().unwrap_or_default();
    let flags_list = flags.iter().cloned().map(|flag| {
        rsx! {
            FlagListItem { flag, reload_flags }
        }
    });

    let create_flag = move |event: Event<FormData>| {
        let mut stores = stores.clone();
        let company = company.clone();
        async move {
            let flag = unwrap_or_report_and_return!(PartialFlag::from_form_data(&event)
                .and_then(|partial| Ok(company.create_flag_from_partial(partial)?)));
            unwrap_or_report_and_return!(stores.store(flag).await);

            // Rerun the resource
            flags_resource.restart();
        }
    };

    rsx! {
        div { id: "flags",

            h3 { "Flags" }

            ul { {flags_list} }

            form { onsubmit: create_flag,
                select { id: "flag_color", name: "flag_color",
                    option { value: "red", "🚩 Red" }
                    option { value: "green", "💚 Green" }
                }
                input { id: "add_flag", name: "name" }
                input { r#type: "submit" }
            }
        }
    }
}
