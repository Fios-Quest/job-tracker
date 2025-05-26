use dioxus::prelude::*;
use std::sync::Arc;
use storage::{ApplicationContext, JsonStores};
use tokio::sync::Mutex;
use ui::Navbar;
use views::{Home, Support};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
    #[route("/support")]
    Support { },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

async fn create_stores() -> JsonStores {
    let directories = directories::ProjectDirs::from("com", "fios-quest", "job-trackers")
        .expect("No valid home directory found!");

    let mut data_dir = directories.data_dir().to_path_buf();

    dbg!(&data_dir);

    data_dir.push("storage");

    JsonStores::new(data_dir)
        .await
        .expect("Could not start database")
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let stores = rt.block_on(create_stores());
    let stores = Arc::new(Mutex::new(stores));

    dioxus::LaunchBuilder::new()
        .with_context(stores)
        .launch(App);
}

#[component]
fn App() -> Element {
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
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Support {}, "Support ❤️" }
        }

        Outlet::<Route> {}
    }
}
