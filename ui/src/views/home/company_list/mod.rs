mod company_list_item;

use crate::helpers::{unwrap_or_report_and_return, CreatePartialFromFormData};
use crate::views::home::company_list::company_list_item::CompanyListItem;
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn CompanyList() -> Element {
    let stores = use_context::<StoreType>();
    let mut company_name_value = use_signal(|| "");
    let mut company_name_search = use_signal(|| "".to_string());

    let mut companies_resource = use_resource(move || async move {
        let search = company_name_search();
        unwrap_or_report_and_return!(use_context::<StoreType>().recall_by_name(search).await)
    });
    let reload_companies = use_callback(move |()| companies_resource.restart());
    let companies = companies_resource().unwrap_or_default();
    let companies_list = companies.into_iter().map(|company| {
        rsx! {
            CompanyListItem { company, reload_companies }
        }
    });

    let create_company = move |event: Event<FormData>| {
        let mut stores = stores.clone();

        async move {
            let partial_company =
                unwrap_or_report_and_return!(PartialCompany::from_form_data(&event));
            let company = unwrap_or_report_and_return!(Company::new_from_partial(partial_company));

            unwrap_or_report_and_return!(stores.store(company).await);

            // Reset the values to empty
            company_name_value.set("");
            company_name_search.set("".to_string());

            // Rerun the resource
            companies_resource.restart();
        }
    };

    let company_search = move |event: Event<FormData>| {
        let search = event.value();
        company_name_search.set(search);
    };

    rsx! {
        div {
            h3 { "Companies" }

            ul { {companies_list} }

            form { class: "flex flex-col", onsubmit: create_company,
                input {
                    id: "add_company",
                    name: "name",
                    value: company_name_search,
                    oninput: company_search,
                }
                input { r#type: "submit" }
            }
        }
    }
}
