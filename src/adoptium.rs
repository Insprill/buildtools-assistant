use std::{env, error::Error, fs, path::PathBuf};

use flate2::read::GzDecoder;
use log::info;
use reqwest::Response;
use serde::Deserialize;
use tar::Archive;

pub async fn get_releases() -> Result<Releases, reqwest::Error> {
    Ok(
        reqwest::get("https://api.adoptium.net/v3/info/available_releases")
            .await?
            .json::<Releases>()
            .await?,
    )
}

#[derive(Deserialize)]
pub struct Releases {
    pub available_releases: Vec<u8>,
}

pub async fn try_download_versions(
    versions: Vec<u8>,
    path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    for java_version in versions {
        let install_path = &path.join(java_version.to_string());
        if install_path.exists() {
            info!("Found existing install for Java {:?}", java_version);
            continue;
        }
        info!("Downloading Java {:?}", java_version);
        download_binaries(java_version, &path.join(java_version.to_string())).await?;
    }
    Ok(())
}

async fn download_binaries(version: u8, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let os = env::consts::OS;
    // todo: detect arm/x86
    let res = reqwest::get(&format!(
        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/x64/jre/hotspot/normal/eclipse",
        version, os
    ))
    .await?;

    let bytes: &[u8] = &res.bytes().await?;
    let mut archive = Archive::new(GzDecoder::new(bytes));
    archive.unpack(path)?;

    Ok(())
}
