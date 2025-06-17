use anyhow::Result;

pub mod json_log_fetcher;

pub trait LogFetcher {
    async fn get_logs(&self) -> Result<Vec<String>>;

    async fn clear_logs(&self) -> Result<()>;

    fn log_location(&self) -> Option<String> {
        None
    }
}
