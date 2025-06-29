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

#[derive(Clone, PartialEq, Debug, Default)]
pub enum DetailsView {
    #[default]
    Company,
    Role,
    Interview(Uuid),
    Questions,
    Invalid,
}

impl FromStr for DetailsView {
    type Err = String; // Never

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(Self::Interview(uuid));
        }
        match s.to_lowercase().as_str() {
            "company" => Ok(Self::Company),
            "role" => Ok(Self::Role),
            "questions" => Ok(Self::Questions),
            _ => Ok(Self::Invalid),
        }
    }
}

impl fmt::Display for DetailsView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DetailsView::Company => write!(f, "company"),
            DetailsView::Role => write!(f, "role"),
            DetailsView::Interview(uuid) => write!(f, "{uuid}"),
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
}
