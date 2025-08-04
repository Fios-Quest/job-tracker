use crate::logging::LogFetcher;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct StubLogFetcher {
    logs: Arc<Mutex<Vec<String>>>,
}

impl StubLogFetcher {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(StubLogFetcher {
            logs: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

impl LogFetcher for StubLogFetcher {
    async fn get_logs(&self) -> anyhow::Result<Vec<String>> {
        Ok(self.logs.lock().await.clone())
    }

    async fn clear_logs(&self) -> anyhow::Result<()> {
        let mut lock = self.logs.lock().await;
        lock.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_logs() {
        // todo!();
        let logger = StubLogFetcher::new().await.unwrap();

        {
            let mut editable = logger.logs.lock().await;
            editable.push("This is a fake log".to_string());
        }

        let logs = logger.get_logs().await.unwrap();
        assert_eq!(logs, vec!["This is a fake log".to_string()]);
    }

    #[tokio::test]
    async fn test_clear_logs() {
        let logger = StubLogFetcher::new().await.unwrap();

        {
            let mut editable = logger.logs.lock().await;
            editable.push("This is a fake log".to_string());
        }

        let logs = logger.get_logs().await.unwrap();
        assert_eq!(logs, vec!["This is a fake log".to_string()]);

        logger.clear_logs().await.unwrap();
        assert_eq!(logs, vec!["This is a fake log".to_string()]);
    }

    #[tokio::test]
    async fn test_log_location() {
        let logger = StubLogFetcher::new().await.unwrap();

        assert!(logger.log_location().is_none());
    }
}
