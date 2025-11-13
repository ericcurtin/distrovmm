use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub config: Descriptor,
    pub layers: Vec<Descriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Descriptor {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: u64,
    pub digest: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

pub struct ImagePuller {
    client: Client,
    registry: String,
}

impl ImagePuller {
    pub fn new(registry: &str) -> Self {
        Self {
            client: Client::new(),
            registry: registry.to_string(),
        }
    }

    /// Parse image reference into repository and tag
    pub fn parse_image_ref(image: &str) -> (String, String) {
        if let Some((repo, tag)) = image.split_once(':') {
            (repo.to_string(), tag.to_string())
        } else {
            (image.to_string(), "latest".to_string())
        }
    }

    /// Get authentication token for Docker Hub
    async fn get_token(&self, repository: &str) -> Result<String> {
        let url = format!(
            "https://auth.docker.io/token?service=registry.docker.io&scope=repository:library/{}:pull",
            repository
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get authentication token")?;
        
        let token_response: TokenResponse = response
            .json()
            .await
            .context("Failed to parse token response")?;
        
        Ok(token_response.token)
    }

    /// Fetch the manifest for an image
    pub async fn fetch_manifest(&self, repository: &str, tag: &str) -> Result<Manifest> {
        let token = self.get_token(repository).await?;
        
        let url = format!(
            "{}/v2/library/{}/manifests/{}",
            self.registry, repository, tag
        );
        
        log::info!("Fetching manifest from: {}", url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.docker.distribution.manifest.v2+json")
            .send()
            .await
            .context("Failed to fetch manifest")?;
        
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to fetch manifest: HTTP {}",
                response.status()
            ));
        }
        
        let manifest: Manifest = response
            .json()
            .await
            .context("Failed to parse manifest")?;
        
        Ok(manifest)
    }

    /// Download a blob (layer or config) from the registry
    pub async fn download_blob(
        &self,
        repository: &str,
        digest: &str,
        output_path: &Path,
    ) -> Result<()> {
        let token = self.get_token(repository).await?;
        
        let url = format!(
            "{}/v2/library/{}/blobs/{}",
            self.registry, repository, digest
        );
        
        log::debug!("Downloading blob: {}", digest);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to download blob")?;
        
        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to download blob: HTTP {}",
                response.status()
            ));
        }
        
        let bytes = response.bytes().await.context("Failed to read blob bytes")?;
        
        let mut file = fs::File::create(output_path)
            .await
            .context("Failed to create output file")?;
        
        file.write_all(&bytes)
            .await
            .context("Failed to write blob to file")?;
        
        Ok(())
    }
}

/// Pull an OCI image and return the path to the downloaded layers
pub async fn pull_image(image: &str, cache_dir: &Path) -> Result<(Manifest, Vec<std::path::PathBuf>)> {
    let (repository, tag) = ImagePuller::parse_image_ref(image);
    
    log::info!("Pulling container image: {}:{}...", repository, tag);
    
    let puller = ImagePuller::new("https://registry-1.docker.io");
    let manifest = puller.fetch_manifest(&repository, &tag).await?;
    
    log::info!("Found {} layers to download", manifest.layers.len());
    
    // Create cache directory for this image
    let image_cache_dir = cache_dir.join(format!("{}_{}", repository, tag));
    fs::create_dir_all(&image_cache_dir)
        .await
        .context("Failed to create cache directory")?;
    
    let mut layer_paths = Vec::new();
    
    for (i, layer) in manifest.layers.iter().enumerate() {
        let layer_file = image_cache_dir.join(format!("layer_{}.tar.gz", i));
        
        if layer_file.exists() {
            log::info!("Layer {}/{} already cached", i + 1, manifest.layers.len());
        } else {
            log::info!("Downloading layer {}/{}", i + 1, manifest.layers.len());
            puller
                .download_blob(&repository, &layer.digest, &layer_file)
                .await?;
        }
        
        layer_paths.push(layer_file);
    }
    
    Ok((manifest, layer_paths))
}
