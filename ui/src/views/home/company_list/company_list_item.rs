use crate::views::home::company_list::forms::edit_company_name::EditCompanyName;
use crate::{Editable, Route};
use application_context::prelude::*;
use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn CompanyListItem(company: Arc<Company>, reload_companies: Callback) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let is_editable = use_signal::<bool>(|| false);

    let company_id = company.id;

    let checked = context()
        .get_company()
        .map(|selected_company| selected_company.id == company.id)
        .unwrap_or(false);
    let display = rsx! {
        input {
            id: "{company_id}",
            r#type: "radio",
            name: "company",
            checked,
            onchange: move |_| {
                spawn(async move {
                    navigator().push(Route::HomeCompany { company_id });
                });
            },
        }
        label { r#for: "{company.id}", "{company.name}" }
    };

    let callback = use_callback(|company: Company| {
        navigator().push(Route::HomeCompany {
            company_id: company.id,
        });
    });

    let editable = rsx! {
        EditCompanyName { company, callback }
    };

    rsx! {
        li { key: "{company_id}",
            Editable { display, editable, is_editable }
        }
    }
}
