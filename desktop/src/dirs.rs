use std::path::PathBuf;

fn get_project_directory() -> PathBuf {
    directories::ProjectDirs::from("com", "fios-quest", "job-trackers")
        .expect("No valid details directory found!")
        .data_dir()
        .to_path_buf()
}

pub fn get_logs_directory() -> PathBuf {
    get_project_directory().join("logs")
}

pub fn get_storage_directory() -> PathBuf {
    get_project_directory().join("storage")
}
