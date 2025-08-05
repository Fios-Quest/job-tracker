use crate::components::ErrorDisplay;
use crate::{Navbar, Route};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "flex mb-4",

            h1 { class: "flex items-center gap-5",
                img {
                    class: "object-scale-down m-auto max-h-[100]",
                    src: asset!("assets/icon.png"),
                }
                "Fio's Job Tracker"
            }

            Navbar {
                Link { to: Route::HomeHome {}, "Home" }
                Link { to: Route::Support {}, "Support ❤️" }
                Link { to: Route::Help {}, "Help" }
            }
        }

        Outlet::<Route> {}

        ErrorDisplay {}
    }
}
