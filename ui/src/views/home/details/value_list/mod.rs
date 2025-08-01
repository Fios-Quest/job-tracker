mod populated_value_list;

use dioxus::events::FormData;
pub use populated_value_list::*;
use std::sync::Arc;
use storage::prelude::{Company, Value};

mod value_list_item;
pub use value_list_item::*;

const VALUE_NAME_FIELD: &str = "value_name";
const VALUE_DESCRIPTION_FIELD: &str = "value_description";

fn form_data_name_and_description(form_data: &FormData) -> Option<(String, String)> {
    let name = form_data.values().get(VALUE_NAME_FIELD)?.as_value();
    let description = form_data
        .values()
        .get(VALUE_DESCRIPTION_FIELD)
        .map(|v| v.as_value())
        .unwrap_or_default();
    Some((name, description))
}

fn create_value_from_form_data(company: Arc<Company>, form_data: &FormData) -> Option<Value> {
    let (name, description) = form_data_name_and_description(form_data)?;
    if name.is_empty() {
        None
    } else {
        Some(company.create_value(name, description))
    }
}

fn edit_value_from_form_data(value: Arc<Value>, form_data: &FormData) -> Option<Value> {
    let (name, description) = form_data_name_and_description(form_data)?;
    if name.is_empty() {
        None
    } else {
        Some(Value::new(value.company_id, name, description))
    }
}
