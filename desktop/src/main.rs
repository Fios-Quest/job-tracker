use dioxus::prelude::*;
use std::fs;
use std::sync::Arc;
use storage::{ApplicationContext, RocksStores};
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

async fn create_rocks() -> RocksStores {
    let directories = directories::ProjectDirs::from("com", "fios-quest", "job-trackers")
        .expect("No valid home directory found!");
    let mut data_dir = directories.data_dir().to_path_buf();
    data_dir.push("database");

    fs::create_dir_all(&data_dir).expect("Couldn't create database directory!");

    // Rocks slightly sucks in that sometimes it doesn't clean up its own lock file 🙄
    let mut lock_file = data_dir.clone();
    lock_file.push("LOCK");
    if lock_file.exists() {
        fs::remove_file(lock_file.as_path()).expect("Failed to remove lock file");
    }

    RocksStores::new(data_dir)
        .await
        .expect("Could not start database")
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let rocks_stores = rt.block_on(create_rocks());
    let stores = Arc::new(Mutex::new(rocks_stores));

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
