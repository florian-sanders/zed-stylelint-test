use std::fs;

pub struct Cache {
    version_dir: String,
}

impl Cache {
    pub fn new(lsp_version: &str) -> Self {
        Self {
            version_dir: format!("lsp-cache-{lsp_version}"),
        }
    }

    /// Returns the directory path for downloads (used by zed::download_file)
    pub fn version_dir(&self) -> &str {
        &self.version_dir
    }

    /// Returns the server script path relative to extension working directory
    pub fn server_path(&self) -> String {
        format!("{}/start-server.js", self.version_dir)
    }

    /// Check if the server file exists using fs::metadata
    /// Note: In WASM sandbox, fs operations work relative to extension's working directory
    pub fn find_cached_build(&self) -> Option<String> {
        let server_path = self.server_path();
        fs::metadata(&server_path)
            .ok()
            .filter(|m| m.is_file())
            .map(|_| server_path)
    }

    /// Remove old version directories to save space
    pub fn cleanup_old_versions(&self) {
        // List common version patterns and remove them if they're not the current version
        // This is a best-effort cleanup since we can't enumerate directories reliably
        let current = &self.version_dir;
        
        // Try to remove a few recent versions (heuristic cleanup)
        for major in 1..=5 {
            for minor in 0..=20 {
                for patch in 0..=10 {
                    let old_dir = format!("lsp-cache-{major}.{minor}.{patch}");
                    if old_dir != *current {
                        let _ = fs::remove_dir_all(&old_dir);
                    }
                }
            }
        }
    }
}
