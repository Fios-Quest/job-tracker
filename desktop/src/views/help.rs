use crate::get_project_directory;
use dioxus::prelude::*;
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

#[component]
pub fn Help() -> Element {
    let project_directory = get_project_directory();
    let project_directory = project_directory.to_string_lossy();
    let href = format!("file://{}", project_directory);

    rsx! {
        h1 { "Help" }
        p {
            "Job Tracker Directory: "
            a { href, {project_directory} }
        }
    }
}
