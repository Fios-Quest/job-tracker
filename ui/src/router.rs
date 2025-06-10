use crate::views::{Help, Home, Support};
use crate::Navbar;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
fn GeneralNavbar() -> Element {
    rsx! {
        Navbar {
            Link { class: "hover:underline", to: Route::HomeHome {}, "Home" }
            Link { class: "hover:underline", to: Route::Support {}, "Support ❤️" }
            Link { class: "hover:underline", to: Route::Help {}, "Help" }
        }

        Outlet::<Route> {}
    }
}

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
    #[layout(GeneralNavbar)]
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
