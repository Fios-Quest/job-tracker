use crate::{Navbar, Route};
use dioxus::prelude::*;

#[component]
pub fn MainNavbar() -> Element {
    rsx! {
        Navbar {
            Link { class: "hover:underline", to: Route::HomeHome {}, "Home" }
            Link { class: "hover:underline", to: Route::Support {}, "Support ❤️" }
            Link { class: "hover:underline", to: Route::Help {}, "Help" }
        }

        Outlet::<Route> {}
    }
}
