use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Combined logger that writes to both file and memory buffer
pub struct CombinedLogger {
    pub buffer: Arc<Mutex<Vec<String>>>,
    pub log_file: Arc<Mutex<std::fs::File>>,
    pub log_file_path: Arc<PathBuf>,
}

impl CombinedLogger {
    /// Creates a new logger with the specified log directory
    pub fn new(log_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let log_file_path = log_path.join(format!("app_{}.log", timestamp));
        let path_for_storage = log_file_path.clone();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;

        Ok(Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            log_file: Arc::new(Mutex::new(file)),
            log_file_path: Arc::new(path_for_storage),
        })
    }
}

impl log::Log for CombinedLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log_message = format!("{} - {}\n", record.level(), record.args());

            if let Ok(mut lock) = self.buffer.lock() {
                lock.push(log_message.trim().to_string());
            }

            if let Ok(mut file) = self.log_file.lock() {
                let _ = file.write_all(log_message.as_bytes());
                let _ = file.flush();
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.log_file.lock() {
            let _ = file.flush();
        }
    }
}
