use crate::{Editable, StoreContext};
use dioxus::prelude::*;
use std::str::FromStr;
use storage::storable::object::flag::{Flag, FlagColor};

#[component]
pub fn FlagListItem(flag: Flag, reload_flags: Callback) -> Element {
    let mut form_receiver: Signal<Option<Event<FormData>>> = use_signal(|| None);
    let stores = use_context::<StoreContext>();

    let input_name = "flag_name";
    let input_color = "flag_color";

    if let Some(event) = form_receiver() {
        let stores = stores.clone();
        let flag = flag.clone();
        let flag_name = event.values().get(input_name).map(|v| v.as_value());
        let flag_color = event
            .values()
            .get(input_color)
            .map(|v| v.as_value())
            .and_then(|v| FlagColor::from_str(&v).ok());
        spawn(async move {
            if let (Some(flag_color), Some(name)) = (flag_color, flag_name) {
                if !name.is_empty() {
                    let flag = Flag {
                        name,
                        flag_color,
                        ..flag
                    };
                    let _result = stores.update_flag(&flag).await;
                    reload_flags(());
                    form_receiver.set(None);
                }
            }
        });
    }

    let Flag {
        id,
        flag_color,
        name,
        ..
    } = flag;
    let flag_icon = match flag_color {
        FlagColor::Green => "💚",
        FlagColor::Red => "🚩",
    };

    let id = id.to_string();

    let display = rsx! { "{flag_icon} {name}" };
    let editable: Element = rsx! {
        select { name: input_color,
            option { selected: flag_color == FlagColor::Red, value: "red", "🚩 Red" }
            option { selected: flag_color == FlagColor::Green, value: "green", "💚 Green" }
        }
        input { name: input_name, value: name }
    };

    rsx! {
        li { id: "flag-{id}",
            if form_receiver().is_none() {
                Editable { display, editable, form_receiver }
            } else {
                "pending"
            }
        }
    }
}
