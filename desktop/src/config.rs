use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

fn window_config() -> WindowBuilder {
    WindowBuilder::new()
        .with_always_on_top(false)
        .with_title("Fio's Job Tracker")
        .with_min_inner_size(LogicalSize::new(800, 600))
        .with_inner_size(LogicalSize::new(1200, 800))
}

pub fn desktop_config() -> Config {
    Config::new().with_window(window_config())
}
