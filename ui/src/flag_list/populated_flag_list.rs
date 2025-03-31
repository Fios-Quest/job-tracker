use super::flag_list_item::FlagListItem;
use crate::StoreContext;
use dioxus::prelude::*;
use std::str::FromStr;
use storage::Store;
use storage::{Flag, FlagColor, FlagStore, Stores};
use uuid::Uuid;

#[component]
pub fn PopulatedFlagList(company_id: Uuid) -> Element {
    let stores = use_context::<StoreContext>();
    let mut flag_name_value = use_signal(|| "");

    // Get flags for company
    let mut flags_resource = use_resource(use_reactive!(|(company_id,)| async move {
        use_context::<StoreContext>()
            .lock()
            .expect("Could not lock flag store")
            .flag_store()
            .get_for_company(company_id)
            .await
            .expect("Did not get flags")
    }));
    let flags = flags_resource().unwrap_or_default();
    let flags_list = flags.iter().cloned().map(|flag| {
        rsx! {
            FlagListItem { flag }
        }
    });

    let create_flag = move |event: Event<FormData>| {
        let stores = stores.clone();
        async move {
            let flag_name = event.values().get("flag_name").map(|v| v.as_value());
            let flag_color = event
                .values()
                .get("flag_color")
                .and_then(|v| FlagColor::from_str(&v.as_value()).ok());

            if let (Some(flag_name), Some(flag_color)) = (flag_name, flag_color) {
                if !flag_name.is_empty() {
                    // Store the name
                    let mut stores_lock = stores.lock().expect("Could not lock flag store");

                    let flag = match flag_color {
                        FlagColor::Green => Flag::new_green(company_id, flag_name),
                        FlagColor::Red => Flag::new_red(company_id, flag_name),
                    };

                    stores_lock
                        .flag_store()
                        .create(flag)
                        .await
                        .expect("Could not store new flag");

                    // Reset the values to empty
                    flag_name_value.set("");

                    // Rerun the resource
                    flags_resource.restart();
                }
            }
        }
    };

    rsx! {
        div {
            id: "flags",
            class: "{company_id}",

            h3 { "Flags" }

            ul {
                { flags_list }
            }

            form {
                onsubmit: create_flag,
                select {
                    id: "flag_color",
                    name: "flag_color",
                    option { value: "red", "🚩 Red" }
                    option { value: "green", "💚 Green" }
                }
                input { id: "add_flag", name: "flag_name", value: flag_name_value }
                input { r#type: "submit" }
            }
        }
    }
}
