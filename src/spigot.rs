use log::info;
use reqwest::{self, Client};

pub async fn versions_exist(versions: &Vec<String>) -> Result<Option<&str>, reqwest::Error> {
    info!("Fetching supported versions");
    let res = Client::new()
        .get("https://hub.spigotmc.org/versions")
        .header("User-Agent", user_agent())
        .send()
        .await?
        .text()
        .await?;
    for version in versions {
        if !res.contains(version) {
            return Ok(Some(version));
        }
    }
    Ok(None)
}

pub async fn download_buildtools() -> Result<Vec<u8>, reqwest::Error> {
    info!("Downloading the latest BuildTools release");
    Ok(Client::new()
    .get("https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar")
    .header("User-Agent", user_agent())
    .send()
    .await?
    .bytes()
    .await?.to_vec())
}

fn user_agent() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
