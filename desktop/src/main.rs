use dioxus::prelude::*;
use storage::{ApplicationContext, JsonStores};
use ui::{Navbar, StoreContext};
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
    let stores_context = StoreContext::new(stores);

    dioxus::LaunchBuilder::new()
        .with_context(stores_context)
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
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
