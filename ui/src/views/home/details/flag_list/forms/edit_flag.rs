use crate::helpers::{log_error, report_if_error};
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::{ApplyPartial, BaseStore, Flag, FlagColor, PartialFlag};

fn create_on_submit(flag: Flag, callback: Callback) -> impl FnMut(FormEvent) -> () {
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<PartialFlag>().map_err(log_error) {
            let mut flag = flag.clone();
            spawn(async move {
                flag.apply(form_data);
                let mut stores = use_context::<StoreType>();
                report_if_error!(stores.store(flag).await);
                callback(());
            });
        }
    }
}

#[component]
pub fn EditFlag(flag: Flag, callback: Callback) -> Element {
    let name = flag.name.clone();
    let flag_color = flag.flag_color;
    rsx! {
        form {
            onsubmit: create_on_submit(flag, callback),
            select { name: "flag_color",
                option { selected: flag_color == FlagColor::Red, value: "red", "🚩 Red" }
                option { selected: flag_color == FlagColor::Green, value: "green", "💚 Green" }
            }
            input { name: "name", value: "{name}" }
            input {
                r#type: "submit",
            }
        }
    }
}
