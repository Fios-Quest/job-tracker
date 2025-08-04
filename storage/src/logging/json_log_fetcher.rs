use crate::logging::LogFetcher;
use anyhow::Result;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs::{read, read_dir, remove_file};

#[derive(Clone)]
pub struct JsonLogFetcher {
    dir: PathBuf,
}

impl JsonLogFetcher {
    pub async fn new(dir: PathBuf) -> Result<Self> {
        tokio::fs::create_dir_all(&dir).await?;
        Ok(Self { dir })
    }
}

impl LogFetcher for JsonLogFetcher {
    async fn get_logs(&self) -> Result<Vec<String>> {
        let mut logs = Vec::new();

        tokio::fs::create_dir_all(&self.dir).await?;
        let mut dir = read_dir(&self.dir).await?;

        while let Some(entry) = dir.next_entry().await? {
            if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("log")) {
                let raw_data = read(&entry.path()).await?;
                let file_data = String::from_utf8_lossy(&raw_data);
                logs.extend(file_data.lines().map(String::from));
            }
        }

        logs.reverse();

        Ok(logs)
    }

    async fn clear_logs(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.dir).await?;
        let mut dir = read_dir(&self.dir).await?;

        while let Some(entry) = dir.next_entry().await? {
            if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("log")) {
                remove_file(entry.path()).await?;
            }
        }
        Ok(())
    }

    fn log_location(&self) -> Option<String> {
        self.dir.to_str().map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn prepare_log() -> PathBuf {
        let dir = tempdir::TempDir::new("log_store_path").unwrap().into_path();
        let log_file = dir.join("log.log");
        let non_log_file = dir.join("log.json");

        // Write a log:
        tokio::fs::write(&log_file, "This is a fake log")
            .await
            .unwrap();
        // Write a file that's not a log:
        tokio::fs::write(&non_log_file, "This should not show in tests")
            .await
            .unwrap();

        dir
    }

    #[tokio::test]
    async fn test_get_logs() {
        let base_path = prepare_log().await;

        let logger = JsonLogFetcher::new(base_path).await.unwrap();

        let logs = logger.get_logs().await.unwrap();
        assert_eq!(logs, vec!["This is a fake log".to_string()]);
    }

    #[tokio::test]
    async fn test_clear_logs() {
        let base_path = prepare_log().await;

        let logger = JsonLogFetcher::new(base_path).await.unwrap();

        let logs = logger.get_logs().await.unwrap();
        assert_eq!(logs, vec!["This is a fake log".to_string()]);

        logger.clear_logs().await.unwrap();
        assert_eq!(logs, vec!["This is a fake log".to_string()]);
    }

    #[tokio::test]
    async fn test_log_location() {
        let base_path = prepare_log().await;

        let logger = JsonLogFetcher::new(base_path).await.unwrap();

        assert!(logger.log_location().is_some());
    }
}
