use std::sync::LazyLock;

use anyhow::Result;
use log::{info, warn};
use reqwest::{self, Client};
use scraper::{Html, Node, Selector};

use crate::USER_AGENT;

static VERSION_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("a[href]").unwrap());

pub async fn versions_exist(versions: &[String]) -> Result<Option<&str>, reqwest::Error> {
    info!("Fetching supported versions");
    for version in versions {
        // if !res.contains(version) {
        //     return Ok(Some(version));
        // }
    }
    Ok(None)
}

pub async fn download_buildtools() -> Result<Vec<u8>, reqwest::Error> {
    info!("Downloading the latest BuildTools release");
    Ok(Client::new()
    .get("https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar")
    .header("User-Agent", USER_AGENT)
    .send()
    .await?
    .bytes()
    .await?.to_vec())
}

pub async fn fetch_all_versions() -> Result<Vec<Version>> {
    let res = Client::new()
        .get("https://hub.spigotmc.org/versions")
        .header("User-Agent", USER_AGENT)
        .send()
        .await?
        .text()
        .await?;
    let mut versions = Vec::new();
    let doc = Html::parse_document(&res);
    for child in doc.select(&VERSION_SELECTOR).flat_map(|e| e.descendants()) {
        let Node::Text(text) = child.value() else {
            continue;
        };

        // Example outputs:
        // 1.8
        // 1.10
        // 1.12.2
        // 1.13-pre7
        // 305
        // 849-a
        // 2219
        let mut text = text.trim_end_matches(".json");

        // Real version: 1.8
        // Fake version: 1556
        // If the second character is a '.', it's real.
        let chars = text.chars().collect::<Vec<_>>();
        if chars.first() != Some(&'1') || chars.get(1) != Some(&'.') {
            continue;
        }

        // Skip pre/rc releases (e.g. '1.18-pre8').
        // There's (currently :D) none of these with patches, so we can just check the 5th char.
        if chars.get(4) == Some(&'-') {
            continue;
        }

        match Version::parse(&text) {
            Ok(version) => versions.push(version),
            Err(err) => {
                warn!("Error parsing version '{text}': {err}. This version will not be available!")
            }
        }
    }
    Ok(versions)
}
