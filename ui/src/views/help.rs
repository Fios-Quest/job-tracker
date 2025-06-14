// use anyhow::Result;
use dioxus::prelude::*;
// use std::ffi::OsStr;
// use tokio::fs::{create_dir_all, read, read_dir, remove_file};
// use std::process::Command;

// #[cfg(target_os = "windows")]
// fn open_project_directory() {
//     let dir = format!(
//         r#""{}""#,
//         get_project_directory().to_string_lossy().as_ref()
//     );
//     Command::new("explorer")
//         .args(["/select,", &dir])
//         .spawn()
//         .unwrap();
// }
//
// #[cfg(target_os = "macos")]
// fn open_project_directory() {
//     Command::new("open")
//         .current_dir(get_project_directory())
//         .args(["-R", "."])
//         .spawn()
//         .unwrap();
// }

// async fn get_logs() -> Result<Vec<String>> {
//     let mut logs = Vec::new();
//     let logs_dir = get_logs_directory();
//
//     create_dir_all(&logs_dir).await?;
//     let mut dir = read_dir(&logs_dir).await?;
//
//     while let Some(entry) = dir.next_entry().await? {
//         if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("log")) {
//             let raw_data = read(&entry.path()).await?;
//             let file_data = String::from_utf8_lossy(&raw_data);
//             logs.extend(file_data.lines().map(String::from));
//         }
//     }
//
//     logs.reverse();
//
//     Ok(logs)
// }
//
// async fn clear_logs() -> Result<()> {
//     let logs_dir = get_logs_directory();
//
//     create_dir_all(&logs_dir).await?;
//     let mut dir = read_dir(&logs_dir).await?;
//
//     while let Some(entry) = dir.next_entry().await? {
//         if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("log")) {
//             remove_file(entry.path()).await?;
//         }
//     }
//     Ok(())
// }

#[component]
pub fn Help() -> Element {
    // let project_directory = get_project_directory();
    // let project_directory = project_directory.to_string_lossy();
    // let href = format!("file://{}", project_directory);
    //
    // let mut logs_resource = use_resource(|| async { get_logs().await.ok() });
    // let logs = logs_resource()
    //     .unwrap_or_default()
    //     .unwrap_or_default()
    //     .into_iter()
    //     .map(|log| {
    //         rsx! {
    //             li { {log} }
    //         }
    //     });
    //
    // rsx! {
    //     h1 { "Help" }
    //     p {
    //         "Job Tracker Directory: "
    //         a { href, {project_directory} }
    //     }
    //
    //     h2 { "Logs" }
    //     if logs.len() > 0 {
    //         button {
    //             onclick: move |_| {
    //                 spawn(async {
    //                     clear_logs().await.expect("Could not clear logs");
    //                 });
    //                 logs_resource.restart();
    //             },
    //             "Clear logs"
    //         }
    //         ol { {logs} }
    //     } else {
    //         "No logs"
    //     }
    // }
    rsx! { "please fix me 😭" }
}
