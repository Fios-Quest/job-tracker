mod company_details;
pub use company_details::*;

mod role_details;
pub use role_details::*;

// mod interview_details;
// pub use interview_details::*;

use crate::router::DetailsView;
use crate::{Navbar, Route};
use dioxus::prelude::*;
use log::error;
use storage::ApplicationContext;

#[component]
pub fn Details(view: Option<DetailsView>) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let company = context().get_company();
    let role = context().get_role();

    let Some(company) = company else {
        return rsx! {
            "Create or choose a company to get started"
        };
    };

    // ToDo: Not happy with this logic, should be a better way to do it 🤔
    let rendered_view = {
        if view == Some(DetailsView::Role) {
            if let Some(role) = role.clone() {
                rsx! { RoleDetails { role } }
            } else {
                error!("Role view used with no role selected");
                rsx! { CompanyDetails { company: company.clone() } }
            }
        } else {
            rsx! { CompanyDetails { company: company.clone() } }
        }
    };

    rsx! {
        div {
            class: "flex flex-col",

            Navbar {
                Link { to: Route::HomeCompany { company_id: company.id }, "Company Details" }
                if let Some(role) = role {
                    Link { to: Route::HomeRole { company_id: company.id, role_id: role.id, view: DetailsView::Role }, "Role Details" }
                    Link { to: Route::Help {}, "Interview Details" }
                    Link { to: Route::Help {}, "Questions" }
                } else {
                    span { class:"disabled-nav-link", "Role Details" }
                    span { class:"disabled-nav-link", "Interview Details" }
                    span { class:"disabled-nav-link", "Questions" }
                }
            }

            section {
                {rendered_view}
            }
        }

    }
}
