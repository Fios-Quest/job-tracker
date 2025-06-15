use anyhow::Result;
use dioxus::prelude::*;
use log::error;
use std::ffi::OsStr;
use std::process::Command;
use std::sync::Arc;
use storage::prelude::{JsonLogFetcher, LogFetcher};
use tokio::fs::{create_dir_all, read, read_dir, remove_file};

#[component]
pub fn Help() -> Element {
    let log_getter = use_context::<JsonLogFetcher>();
    let log_cleaner = use_context::<JsonLogFetcher>();
    let mut logs_resource = use_resource(move || {
        let log_getter = log_getter.clone();
        dbg!("here");
        async move {
            let logs_result = log_getter.get_logs().await;
            logs_result.unwrap_or_else(|e| {
                error!("{}", e);
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
        // p {
        //     "Job Tracker Directory: "
        //     a { href, {project_directory} }
        // }

        h3 { "Logs" }
        if logs.len() > 0 {
            button {
                onclick: move |_| {
                    let log_clearer = log_cleaner.clone();
                    spawn(async move {
                        log_clearer.clear_logs().await.unwrap_or_else(|e| error!("{}", e));
                    });
                    logs_resource.restart();
                },
                "Clear logs"
            }
            ol { {logs} }
        } else {
            "No logs"
        }
    }
}
