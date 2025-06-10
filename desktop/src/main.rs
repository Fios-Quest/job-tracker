use dioxus::prelude::*;
use std::fs::{create_dir_all, OpenOptions};
use std::path::PathBuf;
use storage::prelude::*;
use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use ui::{Route, StoreType};

fn get_project_directory() -> PathBuf {
    directories::ProjectDirs::from("com", "fios-quest", "job-trackers")
        .expect("No valid home directory found!")
        .data_dir()
        .to_path_buf()
}

fn get_logs_directory() -> PathBuf {
    get_project_directory().join("logs")
}

fn get_storage_directory() -> PathBuf {
    get_project_directory().join("storage")
}

async fn create_stores() -> StoreType {
    let path = get_storage_directory();
    JsonThreadSafeGeneralStore::new_json(path)
        .await
        .expect("Could not create store")
}

fn configure_logging() {
    let log_dir = get_logs_directory();

    create_dir_all(&log_dir).expect("Could not create log dir");

    let log_file = log_dir.join("log.log");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
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
    let app_context = ApplicationContext::new();

    dioxus::LaunchBuilder::new()
        .with_context(stores)
        .with_context(app_context)
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/generated/tailwind.css"),
        }
        // Global app resources
        Router::<Route> {}
    }
}
