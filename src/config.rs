use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ExtensionToml {
    pub version: String,
    #[serde(rename = "language_servers")]
    pub language_servers: Option<LanguageServers>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LanguageServers {
    #[serde(rename = "stylelint-lsp")]
    pub stylelint_lsp: Option<StylelintLspConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StylelintLspConfig {
    #[serde(rename = "lsp_required_version")]
    pub lsp_required_version: String,
}

#[derive(Debug, Clone)]
pub struct LspConfig {
    pub extension_version: String,
    pub lsp_version: String,
    pub github_repo: String,
}

impl LspConfig {
    pub fn from_extension_toml() -> Result<Self, String> {
        let content = fs::read_to_string("extension.toml")
            .map_err(|e| format!("Failed to read extension.toml: {}", e))?;

        let parsed: ExtensionToml = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse extension.toml: {}", e))?;

        let lsp_version = parsed
            .language_servers
            .as_ref()
            .and_then(|ls| ls.stylelint_lsp.as_ref())
            .map(|cfg| cfg.lsp_required_version.clone())
            .ok_or("Missing lsp_required_version in extension.toml")?;

        Ok(Self {
            extension_version: parsed.version,
            lsp_version,
            github_repo: "florian-sanders/zed-stylelint".to_string(),
        })
    }

    /// Build the download URL for the LSP zip asset.
    /// The release tag matches the extension version (which defaults to the LSP version).
    pub fn asset_url(&self) -> String {
        format!(
            "https://github.com/{}/releases/download/{}/stylelint-language-server-v{}.zip",
            self.github_repo, self.extension_version, self.lsp_version
        )
    }
}
