use super::flag_list_item::FlagListItem;
use crate::flag_list::forms::create_flag::CreateFlag;
use crate::helpers::unwrap_or_report_and_return;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn PopulatedFlagList(company: Arc<Company>) -> Element {
    let company_id = company.id;

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

    let callback = use_callback(move |_flag_id| flags_resource.restart());

    rsx! {
        div { id: "flags",

            h3 { "Flags" }

            ul { {flags_list} }

            CreateFlag { company, callback }
        }
    }
}
