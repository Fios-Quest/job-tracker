mod populated_value_list;
use dioxus::events::FormData;
pub use populated_value_list::*;
use storage::prelude::Value;
use uuid::Uuid;

mod value_list_item;
pub use value_list_item::*;

const VALUE_NAME_FIELD: &str = "value_name";
const VALUE_DESCRIPTION_FIELD: &str = "value_description";

fn form_date_to_value(company_id: Uuid, form_data: &FormData) -> Option<Value> {
    let name = form_data.values().get(VALUE_NAME_FIELD)?.as_value();
    let description = form_data
        .values()
        .get(VALUE_DESCRIPTION_FIELD)
        .map(|v| v.as_value())
        .unwrap_or_default();
    if name.is_empty() {
        None
    } else {
        Some(Value::new(company_id, name, description))
    }
}
