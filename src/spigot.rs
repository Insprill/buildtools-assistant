use log::info;
use reqwest::{self};

pub async fn download_buildtools() -> Result<Vec<u8>, reqwest::Error> {
    info!("Downloading the latest BuildTools release");
    Ok(reqwest::get("https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar")
    .await?
    .bytes()
    .await?.to_vec())
}
