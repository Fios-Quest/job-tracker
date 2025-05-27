use chrono::Local;
use dioxus::prelude::*;
use directories::ProjectDirs;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;
use storage::{ApplicationContext, JsonStores};
use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;
use ui::{Navbar, StoreContext};
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

fn get_project_directory() -> PathBuf {
    directories::ProjectDirs::from("com", "fios-quest", "job-trackers")
        .expect("No valid home directory found!")
        .data_dir()
        .to_path_buf()
}

async fn create_stores() -> JsonStores {
    log::info!("creating stores");
    let mut data_dir = get_project_directory();
    data_dir.push("storage");

    JsonStores::new(data_dir)
        .await
        .expect("Could not start database")
}

fn configure_logging() {
    let directories = directories::ProjectDirs::from("com", "fios-quest", "job-trackers")
        .expect("No valid home directory found!");

    let mut log_dir = directories.data_dir().to_path_buf();
    log_dir.push("logs");
    create_dir_all(&log_dir).expect("Could not create log dir");

    let date = Local::now();

    let log_file = log_dir.join(format!("{}.log", date.to_rfc3339()));

    let log_file = File::create(log_file)
        .expect("Could not create log file")
        .with_max_level(Level::WARN);

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(log_file);

    Registry::default().with(file_layer).init();
}

fn main() {
    configure_logging();

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
            Link { to: Route::Support {}, "Support ❤️" }
        }

        Outlet::<Route> {}
    }
}
