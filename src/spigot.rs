use log::info;
use reqwest::{self, Client};

pub async fn download_buildtools() -> Result<Vec<u8>, reqwest::Error> {
    info!("Downloading the latest BuildTools release");
    Ok(Client::new()
    .get("https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar")
    .header("User-Agent", env!("CARGO_PKG_NAME"))
    .send()
    .await?
    .bytes()
    .await?.to_vec())
}
