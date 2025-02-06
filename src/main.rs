mod app;
mod logger;
mod task;
mod ui;
mod utils;

use app::{State, APP_DATA_DIR};
use logger::CombinedLogger;

/// Application entry point that sets up logging and runs the UI
fn main() -> iced::Result {
    let logs_dir = APP_DATA_DIR.join("logs");
    let combined_logger = CombinedLogger::new(logs_dir).expect("Could not create logger");
    let log_buffer = combined_logger.buffer.clone();
    let log_path = combined_logger.log_file_path.clone();

    log::set_boxed_logger(Box::new(combined_logger)).expect("Could not set logger");
    log::set_max_level(log::LevelFilter::Info);

    log::info!("App data directory: {}", APP_DATA_DIR.display());

    let initial_state = State {
        log_buffer,
        current_log_path: Some(log_path),
        ..Default::default()
    };

    iced::application("Serendip Whitelister", app::update, ui::view)
        .subscription(app::subscription)
        .theme(|_s: &State| iced::Theme::Dark)
        .run_with(|| (initial_state, iced::Task::none()))
}
