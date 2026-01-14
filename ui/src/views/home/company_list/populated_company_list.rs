use super::company_list_item::CompanyListItem;
use crate::helpers::unwrap_or_report_and_return;
use crate::views::home::company_list::forms::create_company::CreateCompany;
use crate::StoreType;
use dioxus::prelude::*;
use storage::prelude::*;

#[component]
pub fn CompanyList() -> Element {
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

    let callback = use_callback(move |_company_id| {
        // ToDo: Shouldn't the company be selected after creating?

        // Reset search to empty
        company_name_search.set("".to_string());

        // Rerun the resource
        companies_resource.restart();
    });

    let company_search = move |event: Event<FormData>| {
        let search = event.value();
        company_name_search.set(search);
    };

    rsx! {
        div {
            h3 { "Companies" }

            ul { {companies_list} }

            CreateCompany { callback, company_search, company_name_search }
        }
    }
}
