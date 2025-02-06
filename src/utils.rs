use directories::ProjectDirs;
use std::path::{Path, PathBuf};

/// Creates and initializes application directories
pub fn initialize_app_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let proj_dirs = ProjectDirs::from("fi", "helsinki", "serendip-sisu-whitelister")
        .ok_or("Failed to determine project directories")?;

    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;

    let data_subdir = data_dir.join("data");
    let logs_subdir = data_dir.join("logs");

    std::fs::create_dir_all(&data_subdir)?;
    std::fs::create_dir_all(&logs_subdir)?;

    log::info!(
        "Initialized application directory at: {}",
        data_dir.display()
    );
    log::info!("Created data subdirectory at: {}", data_subdir.display());
    log::info!("Created logs subdirectory at: {}", logs_subdir.display());

    Ok(data_dir.to_path_buf())
}

/// Returns a list of CSV files sorted by creation time (newest first)
pub fn get_sorted_csv_files(data_dir: &Path) -> Vec<(PathBuf, std::time::SystemTime)> {
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(data_dir.join("data")) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                if let Ok(metadata) = path.metadata() {
                    if let Ok(created) = metadata.created() {
                        files.push((path, created));
                    }
                }
            }
        }
    }

    files.sort_by(|a, b| b.1.cmp(&a.1));
    files
}
