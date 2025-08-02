//! This crate contains all shared UI for the workspace.
use dioxus::prelude::*;
use storage::prelude::*;

mod helpers;

mod components;
use components::*;

mod router;
use router::*;

mod views;
pub use views::*;

mod layout;

#[cfg(feature = "desktop")]
pub type LogFetcherType = JsonLogFetcher;

#[cfg(all(test, not(feature = "desktop")))]
pub type LogFetcherType = StubLogFetcher;

#[cfg(feature = "desktop")]
pub type StoreType = JsonThreadSafeGeneralStore;

#[cfg(all(test, not(feature = "desktop")))]
pub type StoreType = StubThreadSafeGeneralStore;

pub static SHORTCUT_SIGNAL: GlobalSignal<Option<ShortcutEvent>> = Global::new(|| None);

pub static SHOW_MODIFIERS: GlobalSignal<bool> = Global::new(|| false);

pub mod prelude {
    pub use super::components::ShortcutEvent;
    pub use super::components::ShortcutKey;
    pub use super::components::ShortcutModifier;
    pub use super::router::Route;
    pub use super::LogFetcherType;
    pub use super::StoreType;
    pub use super::SHORTCUT_SIGNAL;
    pub use super::SHOW_MODIFIERS;
}
