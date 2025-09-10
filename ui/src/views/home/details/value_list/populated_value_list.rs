use super::{VALUE_DESCRIPTION_FIELD, VALUE_NAME_FIELD};
use crate::helpers::{unwrap_or_report_and_return, CreatePartialFromFormData};
use crate::value_list::ValueListItem;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, Company, PartialValue, RecallByCompany};

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

    let create_value = move |event: Event<FormData>| {
        let company = company.clone();
        let mut store = use_context::<StoreType>();
        async move {
            let partial_value = unwrap_or_report_and_return!(PartialValue::from_form_data(&event));
            let value =
                unwrap_or_report_and_return!(company.create_value_from_partial(partial_value));
            unwrap_or_report_and_return!(store.store(value).await);
            // Rerun the resource
            values_resource.restart();
        }
    };

    rsx! {
        div { id: "flags",
            h3 { "Values" }
            ul { {values_list} }

            form { onsubmit: create_value,
                input { name: VALUE_NAME_FIELD, value: "" }
                textarea { name: VALUE_DESCRIPTION_FIELD, value: "" }
                input { r#type: "submit" }
            }
        }
    }
}
