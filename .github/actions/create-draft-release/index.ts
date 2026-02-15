import * as core from '@actions/core';
import * as github from '@actions/github';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as crypto from 'crypto';
import { execWithLog } from '../shared/src/exec.js';

async function createZip(sourceDir: string, outputPath: string): Promise<void> {
  // Ensure parent directory exists
  const parentDir = path.dirname(outputPath);
  await fs.mkdir(parentDir, { recursive: true });
  
  // Use system zip command (available on ubuntu-latest)
  core.info(`Creating zip: ${outputPath} from ${sourceDir}`);
  await execWithLog('zip', ['-r', outputPath, '.'], { cwd: sourceDir });
}

async function calculateSHA256(filePath: string): Promise<string> {
  const data = await fs.readFile(filePath);
  return crypto.createHash('sha256').update(data).digest('hex');
}

async function run(): Promise<void> {
  try {
    const lspVersion = core.getInput('lsp-version', { required: true });
    const version = core.getInput('version') || lspVersion;
    const token = core.getInput('github-token', { required: true });
    const isDraft = core.getInput('draft') !== 'false'; // Default true
    const body = core.getInput('body') || `Draft release for LSP v${lspVersion}`;
    
    const octokit = github.getOctokit(token);
    const { owner, repo } = github.context.repo;

    // Create asset file paths
    const zipName = `stylelint-language-server-v${lspVersion}.zip`;
    const sha256Name = `stylelint-language-server-v${lspVersion}.sha256`;
    
    // Verify lsp/ directory exists
    try {
      await fs.access('lsp');
    } catch {
      throw new Error('lsp/ directory not found. Run build-lsp action first.');
    }

    // Create zip archive
    core.info('Creating zip archive...');
    await createZip('lsp', zipName);
    
    // Get file size for logging
    const zipStat = await fs.stat(zipName);
    core.info(`Zip created: ${zipName} (${(zipStat.size / 1024).toFixed(2)} KB)`);
    
    // Calculate SHA256
    core.info('Calculating SHA256 checksum...');
    const sha256 = await calculateSHA256(zipName);
    await fs.writeFile(sha256Name, `${sha256}  ${zipName}\n`);
    core.info(`SHA256: ${sha256}`);

    // Read asset files as binary strings (Octokit types declare data as string,
    // but the runtime sends it as raw binary via application/octet-stream)
    const zipData = await fs.readFile(zipName, 'binary');
    const sha256Data = await fs.readFile(sha256Name, 'utf-8');

    // Try to find existing release
    let release;
    try {
      const { data: existingRelease } = await octokit.rest.repos.getReleaseByTag({
        owner,
        repo,
        tag: version
      });
      
      release = existingRelease;
      core.info(`Found existing release: ${release.html_url}`);
      
      // Check if assets already exist and delete them
      const existingZipAsset = release.assets.find(a => a.name === zipName);
      const existingShaAsset = release.assets.find(a => a.name === sha256Name);
      
      if (existingZipAsset) {
        core.info(`Deleting existing asset: ${zipName}`);
        await octokit.rest.repos.deleteReleaseAsset({
          owner,
          repo,
          asset_id: existingZipAsset.id
        });
      }
      
      if (existingShaAsset) {
        core.info(`Deleting existing asset: ${sha256Name}`);
        await octokit.rest.repos.deleteReleaseAsset({
          owner,
          repo,
          asset_id: existingShaAsset.id
        });
      }
      
    } catch (error: any) {
      // Release doesn't exist, create it
      if (error.status === 404) {
        core.info(`Creating new ${isDraft ? 'draft ' : ''}release v${version}...`);
        const { data: newRelease } = await octokit.rest.repos.createRelease({
          owner,
          repo,
          tag_name: version,
          name: `v${version}`,
          body,
          draft: isDraft,
          prerelease: false
        });
        release = newRelease;
      } else {
        throw error;
      }
    }

    // Upload zip asset
    core.info(`Uploading ${zipName}...`);
    await octokit.rest.repos.uploadReleaseAsset({
      owner,
      repo,
      release_id: release.id,
      name: zipName,
      data: zipData,
      headers: {
        'content-type': 'application/zip'
      }
    });

    // Upload SHA256 asset
    core.info(`Uploading ${sha256Name}...`);
    await octokit.rest.repos.uploadReleaseAsset({
      owner,
      repo,
      release_id: release.id,
      name: sha256Name,
      data: sha256Data,
      headers: {
        'content-type': 'text/plain'
      }
    });

    core.setOutput('release-id', release.id.toString());
    core.setOutput('release-url', release.html_url);
    core.setOutput('upload-url', release.upload_url);
    core.info(`✅ Release ready: ${release.html_url}`);

    // Cleanup local files
    core.info('Cleaning up local files...');
    await fs.unlink(zipName);
    await fs.unlink(sha256Name);
    
  } catch (error) {
    core.setFailed(error instanceof Error ? error.message : 'Unknown error');
  }
}

run();
