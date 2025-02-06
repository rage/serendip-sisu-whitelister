use iced::{Subscription, Task};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::task::{self};
use crate::utils::{get_sorted_csv_files, initialize_app_directory};

/// Global application data directory
pub static APP_DATA_DIR: Lazy<PathBuf> =
    Lazy::new(|| initialize_app_directory().expect("Failed to initialize application directory"));

/// Represents the current state of task execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskState {
    #[default]
    Idle,
    Running,
    Completed,
}

/// Application state containing UI and task execution data
#[derive(Default)]
pub struct State {
    pub actual_progress: f32,
    pub displayed_progress: f32,
    pub task_state: TaskState,
    pub log_buffer: Arc<Mutex<Vec<String>>>,
    pub show_dialog: bool,
    pub current_log_path: Option<Arc<PathBuf>>,
    pub csv_files: Vec<(PathBuf, std::time::SystemTime)>,
    pub show_file_selector: bool,
    pub selected_csv_index: Option<usize>,
    pub show_error: bool,
    pub error_message: String,
    pub shared_progress: Option<Arc<Mutex<f32>>>,
}

/// Messages that can be sent to update the application state
#[derive(Debug, Clone)]
pub enum Message {
    StartPressed,
    OpenLink(String),
    DismissDialog,
    OpenDirectory,
    OpenCurrentLog,
    FileSelected(usize),
    ConfirmFileSelection,
    CancelFileSelection,
    DismissError,
    UpdateProgress(f32),
    Tick,
}

/// Updates application state based on received messages
pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::StartPressed => {
            if state.task_state == TaskState::Idle {
                state.csv_files = get_sorted_csv_files(&APP_DATA_DIR);
                if state.csv_files.is_empty() {
                    state.show_error = true;
                    state.error_message = "No CSV files found in data directory".to_string();
                    log::error!("No CSV files found in data directory");
                } else {
                    state.show_file_selector = true;
                    state.selected_csv_index = Some(0);
                }
            }
        }
        Message::FileSelected(index) => {
            state.selected_csv_index = Some(index);
        }
        Message::ConfirmFileSelection => {
            if let Some(index) = state.selected_csv_index {
                if let Some((selected_file, _)) = state.csv_files.get(index) {
                    log::info!("Selected CSV file: {}", selected_file.display());

                    let file_path = selected_file.clone();
                    state.show_file_selector = false;
                    state.task_state = TaskState::Running;
                    state.actual_progress = 0.0;
                    state.displayed_progress = 0.0;

                    // Create shared progress state
                    let progress = Arc::new(Mutex::new(0.0f32));
                    let progress_for_task = progress.clone();
                    state.shared_progress = Some(progress);

                    // Return a task that will process the file
                    return Task::perform(
                        async move {
                            task::process_csv_file(
                                &file_path,
                                Box::new(move |p| {
                                    log::info!("Progress update: {:.1}%", p * 100.0);
                                    if let Ok(mut current_progress) = progress_for_task.lock() {
                                        *current_progress = p;
                                    }
                                }),
                            )
                        },
                        |result| match result {
                            Ok(_) => Message::UpdateProgress(1.0),
                            Err(error) => {
                                log::error!("Task failed: {}", error);
                                Message::DismissError
                            }
                        },
                    );
                }
            }
        }
        Message::CancelFileSelection => {
            state.show_file_selector = false;
            state.selected_csv_index = None;
        }
        Message::DismissDialog => {
            state.show_dialog = false;
        }
        Message::OpenLink(url) => {
            let _ = open::that(url);
        }
        Message::OpenDirectory => {
            if let Err(e) = open::that(&*APP_DATA_DIR) {
                log::error!("Failed to open directory: {}", e);
            }
        }
        Message::OpenCurrentLog => {
            if let Some(log_path) = &state.current_log_path {
                if let Err(e) = open::that(log_path.as_path()) {
                    log::error!("Failed to open log file: {}", e);
                }
            }
        }
        Message::DismissError => {
            state.show_error = false;
            state.error_message.clear();
        }
        Message::Tick => {
            if state.task_state == TaskState::Running {
                if let Some(progress) = &state.shared_progress {
                    if let Ok(current_progress) = progress.lock() {
                        state.actual_progress = *current_progress;
                        state.displayed_progress = state.actual_progress;
                    }
                }
            }
        }
        Message::UpdateProgress(progress) => {
            state.actual_progress = progress;
            state.shared_progress = None;

            if progress >= 1.0 {
                state.task_state = TaskState::Completed;
                state.displayed_progress = 1.0;
                state.show_dialog = true;
                log::info!("Task completed successfully!");
            }
        }
    }
    Task::none()
}

/// Returns a subscription for progress updates when task is running
pub fn subscription(state: &State) -> Subscription<Message> {
    if state.task_state == TaskState::Running {
        iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    } else {
        Subscription::none()
    }
}
