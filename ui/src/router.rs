use crate::views::{Help, Home, Support};
use crate::MainNavbar;
use dioxus::prelude::*;
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
fn HomeRole(company_id: Uuid, role_id: Uuid) -> Element {
    rsx! {
        Home { company_id, role_id }
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
    #[route("/:company_id/:role_id")]
    HomeRole { company_id: Uuid, role_id: Uuid },
}
