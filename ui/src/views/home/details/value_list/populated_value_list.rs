use crate::helpers::unwrap_or_report_and_return;
use crate::value_list::forms::create_value::CreateValue;
use crate::value_list::ValueListItem;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{Company, RecallByCompany};

#[component]
pub fn ValueList(company: Arc<Company>) -> Element {
    let company_id = company.id;

    let mut values_resource = use_resource(use_reactive!(|(company_id)| async move {
        let values = unwrap_or_report_and_return!(
            use_context::<StoreType>()
                .recall_by_company(company_id)
                .await
        );

        values.into_iter().map(Arc::new).collect::<Vec<_>>()
    }));

    let reload_values = use_callback(move |()| values_resource.restart());
    let values = values_resource().unwrap_or_default();
    let values_list = values.iter().cloned().map(move |value| {
        rsx! {
            ValueListItem { value, reload_values }
        }
    });

    let callback = use_callback(move |_value| values_resource.restart());

    rsx! {
        div { id: "flags",
            h3 { "Values" }
            ul { {values_list} }

            CreateValue { company, callback }
        }
    }
}
