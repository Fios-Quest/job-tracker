mod company_details;
pub use company_details::*;

mod questions_details;
pub use questions_details::*;

mod role_details;
pub use role_details::*;

mod interview_details;
pub use interview_details::*;

use crate::router::{create_route, DetailsView};
use crate::Navbar;
use dioxus::prelude::*;
use log::info;
use storage::ApplicationContext;

#[component]
fn InnerDetailView(view: DetailsView) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let company = context().get_company();
    let role = context().get_role();

    // At the very least, Company must exist
    let Some(company) = company else {
        return rsx! { "Create or choose a company to get started" };
    };

    match view {
        DetailsView::Company => {
            rsx! {
                CompanyDetails { company }
            }
        }
        DetailsView::Role => {
            if let Some(role) = role {
                rsx! {
                    RoleDetails { role }
                }
            } else {
                info!("Role view used with no role selected");
                rsx! {
                    CompanyDetails { company }
                }
            }
        }
        DetailsView::Interview => {
            if let Some(role) = role {
                rsx! {
                    InterviewDetails { role }
                }
            } else {
                info!("Interview view used with no role selected");
                rsx! {
                    CompanyDetails { company }
                }
            }
        }
        DetailsView::Questions => {
            if let Some(role) = role {
                rsx! {
                    QuestionsDetails { role }
                }
            } else {
                info!("Question view used with no role selected");
                rsx! {
                    CompanyDetails { company }
                }
            }
        }
        DetailsView::Invalid => {
            rsx! {
                CompanyDetails { company }
            }
        }
    }
}

#[component]
pub fn Details(view: Option<DetailsView>) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let route_to_view = |view: DetailsView| {
        create_route(
            context().get_company().map(|c| c.id),
            context().get_role().map(|r| r.id),
            context().get_interview().map(|i| i.id),
            Some(view),
        )
    };

    rsx! {
        div { class: "flex flex-col",

            Navbar {
                Link { to: route_to_view(DetailsView::Company), "Company Details" }
                if context().get_role().is_some() {
                    Link { to: route_to_view(DetailsView::Role), "Role Details" }
                    Link { to: route_to_view(DetailsView::Interview), "Interview Details" }
                    Link { to: route_to_view(DetailsView::Questions), "Questions" }
                } else {
                    span { class: "disabled-nav-link", "Role Details" }
                    span { class: "disabled-nav-link", "Interview Details" }
                    span { class: "disabled-nav-link", "Questions" }
                }
            }

            section {
                InnerDetailView { view: view.unwrap_or(DetailsView::Company) }
            }
        }
    }
}
