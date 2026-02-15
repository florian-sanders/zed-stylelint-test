#[derive(Debug, Clone)]
pub struct LspConfig {
    pub extension_version: String,
    pub lsp_version: String,
    pub github_repo: String,
}

impl LspConfig {
    /// Extension and LSP version constants.
    /// Keep these in sync with extension.toml when bumping versions.
    const EXTENSION_VERSION: &str = "2.0.2";
    const LSP_VERSION: &str = "2.0.2";
    const GITHUB_REPO: &str = "florian-sanders/zed-stylelint";

    pub fn new() -> Self {
        Self {
            extension_version: Self::EXTENSION_VERSION.to_string(),
            lsp_version: Self::LSP_VERSION.to_string(),
            github_repo: Self::GITHUB_REPO.to_string(),
        }
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
