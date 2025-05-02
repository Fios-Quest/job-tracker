use dioxus::prelude::*;
use std::fs;
use std::sync::Arc;
use storage::{ApplicationContext, LibSqlStores};
use tokio::sync::Mutex;
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

async fn create_libsql() -> LibSqlStores {
    let directories = directories::ProjectDirs::from("com", "fios-quest", "job-trackers")
        .expect("No valid home directory found!");

    let mut data_dir = directories.data_dir().to_path_buf();
    fs::create_dir_all(&data_dir).expect("Couldn't create data directory!");

    data_dir.push("database.db");

    dbg!(&data_dir);

    LibSqlStores::new(data_dir)
        .await
        .expect("Could not start database")
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let libsql_stores = rt.block_on(create_libsql());
    let stores = Arc::new(Mutex::new(libsql_stores));

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
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
