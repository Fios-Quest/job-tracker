#[path = "empty_flag_list.rs"]
mod empty_flag_list;
mod flag_list_item;
mod populated_flag_list;

use dioxus::prelude::*;
use empty_flag_list::EmptyFlagList;
use populated_flag_list::PopulatedFlagList;
use uuid::Uuid;

#[component]
pub fn FlagList(company_id: Option<Uuid>) -> Element {
    match company_id {
        Some(company_id) => rsx! {
            PopulatedFlagList { company_id }
        },
        None => rsx! {
            EmptyFlagList {}
        },
    }
}
