use crate::LogFetcherType;
use dioxus::prelude::*;
use log::error;
use std::process::Command;
use storage::prelude::*;

#[cfg(all(target_os = "windows", feature = "desktop"))]
fn open_dir(path: &str) -> anyhow::Result<()> {
    Command::new("explorer").arg(path).spawn()?.wait()?;
    Ok(())
}

#[cfg(all(target_os = "macos", feature = "desktop"))]
fn open_dir(path: &str) -> anyhow::Result<()> {
    Command::new("open").arg(path).spawn()?.wait()?;
    Ok(())
}

#[cfg(all(target_os = "linux", feature = "desktop"))]
fn open_dir(path: &str) -> anyhow::Result<()> {
    Command::new("xdg-open").arg(path).spawn()?.wait()?;
    Ok(())
}

#[component]
pub fn Help() -> Element {
    let log_getter = use_context::<LogFetcherType>();
    let log_cleaner = use_context::<LogFetcherType>();
    let mut logs_resource = use_resource(move || {
        let log_getter = log_getter.clone();
        async move {
            let logs_result = log_getter.get_logs().await;
            logs_result.unwrap_or_else(|e| {
                error!("{e}");
                Vec::with_capacity(0)
            })
        }
    });

    let logs = logs_resource().unwrap_or_default().into_iter().map(|log| {
        rsx! {
            li { {log} }
        }
    });

    rsx! {
        h2 { "Help" }

        if let Some(path) = log_cleaner.log_location() {
            p {
                "Log Directory: "
                a {
                    href: "#",
                    onclick: move |_| { open_dir(&path).expect("Could not open dir") },
                    {path.clone()}
                }
            }
        }

        h3 { "Logs" }
        if logs.len() > 0 {
            button {
                onclick: move |_| {
                    let log_clearer = log_cleaner.clone();
                    spawn(async move {
                        log_clearer.clear_logs().await.unwrap_or_else(|e| error!("{e}"));
                        logs_resource.restart();
                    });
                },
                "Clear logs"
            }
            ol { {logs} }
        } else {
            "No logs"
        }
    }
}
