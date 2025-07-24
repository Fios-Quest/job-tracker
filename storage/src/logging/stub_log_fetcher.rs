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
