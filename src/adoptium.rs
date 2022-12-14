use std::{
    error::Error,
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use log::info;
use serde::Deserialize;
use tar::Archive;

use crate::os::OS;

pub async fn get_releases() -> Result<Releases, reqwest::Error> {
    reqwest::get("https://api.adoptium.net/v3/info/available_releases")
        .await?
        .json::<Releases>()
        .await
}

#[derive(Deserialize)]
pub struct Releases {
    pub available_releases: Vec<u8>,
    pub available_lts_releases: Vec<u8>,
}

pub async fn try_download_versions(
    releases: &Releases,
    versions: Vec<u8>,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    for java_version in versions {
        let install_path = &path.join(java_version.to_string());
        if install_path.exists() && install_path.read_dir()?.next().is_some() {
            info!("Found existing install for Java {:?}", java_version);
            continue;
        }
        info!("Downloading Java {:?}", java_version);
        download_binaries(releases, java_version, &path.join(java_version.to_string())).await?;
    }
    Ok(())
}

async fn download_binaries(
    releases: &Releases,
    version: u8,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let os = OS::current();
    let image_type = if releases.available_lts_releases.contains(&version) {
        "jre"
    } else {
        "jdk"
    };
    // todo: detect arm/x86
    let res = reqwest::get(&format!(
        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/x64/{}/hotspot/normal/eclipse",
        version,
        os.adoptium_name(),
        image_type
    ))
    .await?;

    let bytes: &[u8] = &res.bytes().await?;

    fs::create_dir_all(path)?;

    info!("Extracting Java {:?}", version);
    if os == OS::WINDOWS {
        zip_extract::extract(Cursor::new(bytes), path, false)?;
    } else {
        Archive::new(GzDecoder::new(bytes)).unpack(path)?;
    };

    Ok(())
}

pub async fn get_java_install(version: u8, root_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let version_path = root_path.join(version.to_string());
    assert!(version_path.exists());
    Ok(OS::current().java_dir(&version_path.read_dir()?.next().unwrap()?.path()))
}
