use crate::views::{Help, Home, Support};
use crate::MainNavbar;
use dioxus::prelude::*;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[component]
fn HomeHome() -> Element {
    rsx! {
        Home {}
    }
}

#[component]
fn HomeCompany(company_id: Uuid) -> Element {
    rsx! {
        Home { company_id }
    }
}

#[component]
fn HomeRole(company_id: Uuid, role_id: Uuid, view: DetailsView) -> Element {
    rsx! {
        Home { company_id, role_id, view }
    }
}

#[component]
fn HomeInterview(
    company_id: Uuid,
    role_id: Uuid,
    interview_id: Uuid,
    view: DetailsView,
) -> Element {
    rsx! {
        Home {
            company_id,
            role_id,
            interview_id,
            view,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum DetailsView {
    #[default]
    Company,
    Role,
    Interview,
    Questions,
    Invalid,
}

impl FromStr for DetailsView {
    type Err = String; // Never

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "company" => Ok(Self::Company),
            "role" => Ok(Self::Role),
            "questions" => Ok(Self::Questions),
            "interview" => Ok(Self::Interview),
            _ => Ok(Self::Invalid),
        }
    }
}

impl fmt::Display for DetailsView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DetailsView::Company => write!(f, "company"),
            DetailsView::Role => write!(f, "role"),
            DetailsView::Interview => write!(f, "interview"),
            DetailsView::Questions => write!(f, "questions"),
            DetailsView::Invalid => write!(f, "company"),
        }
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainNavbar)]
    #[route("/support")]
    Support { },
    #[route("/help")]
    Help { },
    #[route("/")]
    HomeHome,
    #[route("/:company_id")]
    HomeCompany { company_id: Uuid },
    #[route("/:company_id/:role_id?:view")]
    HomeRole { company_id: Uuid, role_id: Uuid, view: DetailsView },
    #[route("/:company_id/:role_id/:interview_id?:view")]
    HomeInterview { company_id: Uuid, role_id: Uuid, interview_id: Uuid, view: DetailsView },
}

pub fn create_route(
    company_id: Option<Uuid>,
    role_id: Option<Uuid>,
    interview_id: Option<Uuid>,
    view: Option<DetailsView>,
) -> String {
    let mut route = String::new();
    if let Some(company_id) = company_id {
        route.push('/');
        route.push_str(&company_id.to_string());

        if let Some(role_id) = role_id {
            route.push('/');
            route.push_str(&role_id.to_string());

            if let Some(interview_id) = interview_id {
                route.push('/');
                route.push_str(&interview_id.to_string());
            }
        }
    }

    if let Some(view) = view {
        route.push_str(&format!("?view={view}"))
    }

    route
}
