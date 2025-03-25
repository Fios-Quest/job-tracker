use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use storage::{ApplicationContext, Stores, StubCompanyStore, StubFlagStore, StubRoleStore};
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
    let company_store = StubCompanyStore::new();
    let role_store = StubRoleStore::new();
    let flag_store = StubFlagStore::new();
    let stores = Arc::new(Mutex::new(Stores::new(
        company_store,
        role_store,
        flag_store,
    )));
    use_context_provider(move || stores);

    let application_context = Signal::new(ApplicationContext::new());
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
