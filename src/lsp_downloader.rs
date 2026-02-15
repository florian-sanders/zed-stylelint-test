use std::fs;

use zed_extension_api::{self as zed, LanguageServerId};

use crate::cache::Cache;
use crate::config::LspConfig;

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

        self.download_and_extract(cache.version_dir())
            .map_err(|e| DownloadError::Network {
                url: self.config.asset_url(),
                message: e,
            })?;

        // Verify server file exists
        let server_path = cache.server_path();
        if !fs::metadata(&server_path).map_or(false, |m| m.is_file()) {
            return Err(DownloadError::Extraction {
                message: format!("Server file not found at {} after extraction", server_path),
            });
        }

        // Cleanup old versions to save space
        cache.cleanup_old_versions();

        Ok(server_path)
    }

    fn download_and_extract(&self, destination: &str) -> Result<(), String> {
        let url = self.config.asset_url();

        zed::download_file(&url, destination, zed::DownloadedFileType::GzipTar)
            .map_err(|e| format!("Download failed: {}", e))?;

        Ok(())
    }
}
