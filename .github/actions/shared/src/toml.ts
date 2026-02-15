import * as fs from 'fs/promises';
import toml from 'toml';

export interface ExtensionToml {
  version: string;
  language_servers?: {
    'stylelint-lsp'?: {
      lsp_required_version?: string;
    };
  };
}

export async function readExtensionToml(path = 'extension.toml'): Promise<ExtensionToml> {
  const content = await fs.readFile(path, 'utf-8');
  return toml.parse(content) as ExtensionToml;
}

export async function writeExtensionToml(data: ExtensionToml, path = 'extension.toml'): Promise<void> {
  let tomlString = `version = "${data.version}"\n\n`;
  
  if (data.language_servers?.['stylelint-lsp']?.lsp_required_version) {
    tomlString += '[language_servers.stylelint-lsp]\n';
    tomlString += `lsp_required_version = "${data.language_servers['stylelint-lsp'].lsp_required_version}"\n`;
  }
  
  await fs.writeFile(path, tomlString, 'utf-8');
}
