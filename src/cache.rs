use std::fs;
use std::path::{Path, PathBuf};

pub struct Cache {
    cache_dir: PathBuf,
    lsp_version: String,
}

impl Cache {
    pub fn new(lsp_version: &str) -> Result<Self, String> {
        // Use relative path within extension's working directory
        let cache_dir = PathBuf::from("lsp-cache").join(lsp_version);

        // Ensure cache directory exists
        fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;

        Ok(Self {
            cache_dir,
            lsp_version: lsp_version.to_string(),
        })
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn dist_dir(&self) -> PathBuf {
        self.cache_dir.join("dist")
    }

    pub fn server_path(&self) -> PathBuf {
        self.dist_dir().join("start-server.js")
    }

    pub fn find_cached_build(&self) -> Option<String> {
        let server_path = self.server_path();

        fs::metadata(&server_path)
            .ok()
            .filter(|m| m.is_file())
            .map(|_| server_path.to_string_lossy().to_string())
    }

    pub fn cleanup_old_versions(&self) -> Result<(), String> {
        let parent = self
            .cache_dir
            .parent()
            .ok_or("Could not get cache parent directory")?;

        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    // Skip current version
                    if name != self.lsp_version {
                        let _ = fs::remove_dir_all(entry.path());
                    }
                }
            }
        }

        Ok(())
    }
}
