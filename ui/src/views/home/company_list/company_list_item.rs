use crate::views::home::company_list::forms::edit_company_name::EditCompanyName;
use crate::{Editable, Route};
use application_context::prelude::*;
use dioxus::prelude::*;
use storage::prelude::*;
use uuid::Uuid;

#[component]
pub fn CompanyListItem(company: Company, reload_companies: Callback) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let is_editable = use_signal::<bool>(|| false);

    let id = company.id;
    let name = company.name.clone();

    let checked = context()
        .get_company()
        .map(|selected_company| selected_company.id == company.id)
        .unwrap_or(false);
    let display = rsx! {
        input {
            id: "{id}",
            r#type: "radio",
            name: "company",
            checked,
            onchange: move |_| {
                spawn(async move {
                    navigator()
                        .push(Route::HomeCompany {
                            company_id: company.id,
                        });
                });
            },
        }
        label { r#for: "{id}", "{name}" }
    };

    let callback = use_callback(|company_id: Uuid| {
        navigator().push(Route::HomeCompany { company_id });
    });

    let editable = rsx! {
        EditCompanyName { company, callback }
    };

    rsx! {
        li { key: "{id}",
            Editable { display, editable, is_editable }
        }
    }
}
