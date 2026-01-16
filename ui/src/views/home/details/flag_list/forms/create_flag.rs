use crate::helpers::log_error;
use crate::StoreType;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::{BaseStore, Company, Flag, FlagFieldName, PartialFlag};

fn create_on_submit(company: Arc<Company>, callback: Callback<Flag>) -> impl FnMut(FormEvent) {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(flag) = e
            .parsed_values::<PartialFlag>()
            .map_err(log_error)
            .and_then(|form_data| {
                company
                    .create_flag_from_partial(form_data)
                    .map_err(log_error)
            })
        {
            spawn(async move {
                let mut stores = use_context::<StoreType>();
                stores.store(flag.clone()).await.unwrap_or_else(log_error);
                callback(flag);
            });
        }
    }
}

#[component]
pub fn CreateFlag(company: Arc<Company>, callback: Callback<Flag>) -> Element {
    rsx! {
        form { onsubmit: create_on_submit(company, callback),
            select { id: "flag_color", name: FlagFieldName::FlagColor.name(),
                option { value: "red", "🚩 Red" }
                option { value: "green", "💚 Green" }
            }
            input { id: "add_flag", name: FlagFieldName::Name.name() }
            input { r#type: "submit" }
        }
    }
}
