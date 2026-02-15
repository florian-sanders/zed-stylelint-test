mod cache;
mod config;
mod lsp_downloader;

use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result};

use crate::cache::Cache;
use crate::config::LspConfig;
use crate::lsp_downloader::LspDownloader;

pub struct StylelintExtension {
    config: Option<LspConfig>,
}

impl Default for StylelintExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl StylelintExtension {
    pub fn new() -> Self {
        Self { config: None }
    }

    fn ensure_config(&mut self) -> &LspConfig {
        if self.config.is_none() {
            self.config = Some(LspConfig::new());
        }
        self.config.as_ref().unwrap()
    }

    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let config = self.ensure_config();
        let cache = Cache::new(&config.lsp_version);

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        if let Some(cached_path) = cache.find_cached_build() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::None,
            );
            return Ok(cached_path);
        }

        let downloader = LspDownloader::new(config.clone());
        let server_path = downloader
            .download_and_verify(&cache, language_server_id)
            .map_err(|e| e.to_string())?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(server_path)
    }
}

impl zed::Extension for StylelintExtension {
    fn new() -> Self {
        Self::new()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(settings))
    }
}

zed_extension_api::register_extension!(StylelintExtension);
