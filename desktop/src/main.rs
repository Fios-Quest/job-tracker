use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use storage::{ApplicationContext, StubCompanyStore, StubFlagStore, StubRoleStore};
use ui::Navbar;
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let company_store = Arc::new(Mutex::new(StubCompanyStore::new()));
    let role_store = Arc::new(Mutex::new(StubRoleStore::new()));
    let flag_store = Arc::new(Mutex::new(StubFlagStore::new()));
    let application_context = Signal::new(ApplicationContext::new());
    use_context_provider(move || company_store);
    use_context_provider(move || role_store);
    use_context_provider(move || flag_store);
    use_context_provider(move || application_context);

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
