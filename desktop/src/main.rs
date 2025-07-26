use crate::keyboard::create_keyboard_event_loop;
use dioxus::desktop::window;
use dioxus::prelude::*;
use storage::prelude::*;
use tokio::join;
use ui::{DetailsView, Route, StoreType, VIEW_SIGNAL};

mod config;
mod dirs;
mod keyboard;
mod logs;

async fn create_stores() -> StoreType {
    let path = dirs::get_storage_directory();
    JsonThreadSafeGeneralStore::new_json(path)
        .await
        .expect("Could not create store")
}

async fn create_log_fetcher() -> JsonLogFetcher {
    let path = dirs::get_logs_directory();
    JsonLogFetcher::new(path)
        .await
        .expect("Could not create log fetcher")
}

fn main() {
    logs::configure_logging();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (stores, log_fetcher) = rt.block_on(async { join!(create_stores(), create_log_fetcher()) });

    LaunchBuilder::new()
        .with_context(stores)
        .with_context(log_fetcher)
        .with_cfg(config::desktop_config())
        .launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(ApplicationContext::new()));
    let goto_company = use_callback(|()| {
        *VIEW_SIGNAL.write() = Some(DetailsView::Company);
    });
    let goto_role = use_callback(|()| {
        *VIEW_SIGNAL.write() = Some(DetailsView::Role);
    });

    create_keyboard_event_loop();

    let _ = window()
        .create_shortcut(
            "alt+c".try_into().expect("Could not make hotkey"),
            move || {
                dbg!("Alt+C was pressed");
                goto_company(());
            },
        )
        .expect("Could not create shortcut");
    let _ = window()
        .create_shortcut(
            "alt+r".try_into().expect("Could not make hotkey"),
            move || {
                dbg!("Alt+R was pressed");
                goto_role(());
            },
        )
        .expect("Could not create shortcut");

    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/generated/tailwind.css"),
        }
        div { class: "to-do-when-modifier-works",

            // Global app resources
            Router::<Route> {}
        }
    }
}
