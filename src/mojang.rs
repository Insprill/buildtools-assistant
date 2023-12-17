use anyhow::{bail, Result};
use futures::future;
use itertools::Itertools;
use log::info;
use serde::Deserialize;

async fn fetch_version_manifest() -> Result<VersionManifest, reqwest::Error> {
    info!("Fetching version manifest");
    reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .await?
        .json::<VersionManifest>()
        .await
}

async fn fetch_package(manifest: &Manifest) -> Result<Package, reqwest::Error> {
    info!("Fetching package for {}", manifest.id);
    reqwest::get(&manifest.url).await?.json::<Package>().await
}

#[derive(Deserialize, Debug)]
struct VersionManifest {
    versions: Vec<Manifest>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Manifest {
    pub id: String,
    url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub java_version: JavaVersion,
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub major_version: u8,
}

pub async fn map_version_manifests(versions: &Vec<String>) -> Result<Vec<Manifest>> {
    let mut manifests = Vec::with_capacity(versions.len());

    let version_manifest = fetch_version_manifest().await?;
    for version in versions.iter().unique() {
        let manifest = version_manifest.versions.iter().find(|v| &v.id == version);
        if let Some(m) = manifest {
            manifests.push(m.clone());
        } else {
            bail!("Invalid version {version:?}");
        }
    }
    Ok(manifests)
}

pub async fn fetch_packages(manifests: Vec<Manifest>) -> Result<Vec<Package>> {
    let mut package_handles = Vec::with_capacity(manifests.len());
    for manifest in manifests {
        package_handles.push(tokio::spawn(async move { fetch_package(&manifest).await }));
    }
    let results = future::join_all(package_handles).await;
    let mut packages = Vec::with_capacity(results.len());
    for package in results {
        packages.push(package??);
    }
    Ok(packages)
}
