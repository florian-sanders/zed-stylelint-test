use crate::cache::Cache;
use crate::config::LspConfig;
use std::path::Path;
use zed_extension_api::{self as zed, LanguageServerId};

pub struct LspDownloader {
    config: LspConfig,
}

#[derive(Debug)]
pub enum DownloadError {
    Network { url: String, message: String },
    Extraction { message: String },
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::Network { url, message } => {
                write!(f, "Failed to download from {}: {}", url, message)
            }
            DownloadError::Extraction { message } => {
                write!(f, "Failed to extract language server: {}", message)
            }
        }
    }
}

impl LspDownloader {
    pub fn new(config: LspConfig) -> Self {
        Self { config }
    }

    pub fn download_and_verify(
        &self,
        cache: &Cache,
        language_server_id: &LanguageServerId,
    ) -> Result<String, DownloadError> {
        // Download and extract zip file
        // zed::download_file with DownloadedFileType::Zip automatically extracts
        // the contents into the destination directory
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        self.download_and_extract(cache.cache_dir())
            .map_err(|e| DownloadError::Network {
                url: self.config.asset_url(),
                message: e,
            })?;

        // Verify server file exists
        let server_path = cache.server_path();
        if !server_path.exists() {
            return Err(DownloadError::Extraction {
                message: format!(
                    "Server file not found at {:?} after extraction",
                    server_path
                ),
            });
        }

        // Cleanup old versions to save space
        let _ = cache.cleanup_old_versions();

        Ok(server_path.to_string_lossy().to_string())
    }

    fn download_and_extract(&self, destination: &Path) -> Result<(), String> {
        let url = self.config.asset_url();
        let dest_str = destination.to_string_lossy().to_string();

        zed::download_file(&url, &dest_str, zed::DownloadedFileType::Zip)
            .map_err(|e| format!("Download failed: {}", e))?;

        Ok(())
    }
}
