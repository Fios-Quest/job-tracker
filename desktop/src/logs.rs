use crate::dirs::*;
use std::fs::{create_dir_all, OpenOptions};
use tracing::Level;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

pub fn configure_logging() {
    let log_dir = get_logs_directory();

    create_dir_all(&log_dir).expect("Could not create log dir");

    let log_file = log_dir.join("log.log");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .expect("Could not create log file")
        .with_max_level(Level::WARN);

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(log_file);

    Registry::default().with(file_layer).init();
}
