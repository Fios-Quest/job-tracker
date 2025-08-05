mod company_details;
pub use company_details::*;

mod questions_details;
pub use questions_details::*;

mod role_details;
pub use role_details::*;

pub mod flag_list;
mod interview_details;
pub mod interviews;
pub mod questions_list;
pub mod role_information;
pub mod value_list;

pub use interview_details::*;

use crate::helpers::create_route;
use crate::router::DetailsView;
use crate::{Navbar, ShortcutEvent, ShortcutHelper};
use application_context::prelude::*;
use dioxus::prelude::*;
use log::info;

#[component]
fn InnerDetailView(view: DetailsView) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let company = context().get_company();
    let role = context().get_role();

    // At the very least, the Company must exist
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
fn DetailsNavigation(
    event: ShortcutEvent,
    view: DetailsView,
    children: Element,
    active: bool,
) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let route = create_route(
        context().get_company().map(|c| c.id),
        context().get_role().map(|r| r.id),
        context().get_interview().map(|i| i.id),
        Some(view),
    );
    let route_clone = route.clone();
    let route_callback = use_callback(move |()| {
        navigator().push(route_clone.as_str());
    });

    let class = if active { "active-nav-link" } else { "" };

    rsx! {
        ShortcutHelper { shortcut_event: event, on_shortcut: route_callback,
            Link { class, to: route, {children} }
        }
    }
}

#[component]
pub fn Details(view: Option<DetailsView>) -> Element {
    let context = use_context::<Signal<ApplicationContext>>();

    rsx! {
        div { class: "details",

            Navbar {
                DetailsNavigation {
                    event: ShortcutEvent::company(),
                    view: DetailsView::Company,
                    active: view == Some(DetailsView::Company) || view.is_none(),
                    "Company Details"
                }
                if context().get_role().is_some() {
                    DetailsNavigation {
                        event: ShortcutEvent::role(),
                        view: DetailsView::Role,
                        active: view == Some(DetailsView::Role),
                        "Role Details"
                    }
                    DetailsNavigation {
                        event: ShortcutEvent::interview(),
                        view: DetailsView::Interview,
                        active: view == Some(DetailsView::Interview),

                        "Interview Details"
                    }
                    DetailsNavigation {
                        event: ShortcutEvent::questions(),
                        view: DetailsView::Questions,
                        active: view == Some(DetailsView::Questions),
                        "Questions"
                    }
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
