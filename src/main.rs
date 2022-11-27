use std::error::Error;
use std::process;

use log::{error, LevelFilter};

use clap::{arg, command, Parser};
use serde::Deserialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What versions to run BuildTools for
    versions: Vec<String>,

    /// Whether to also build the Mojang-mapped versions
    #[arg(short, long)]
    remapped: bool,
}

#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let args = Args::parse();

    println!("versions: {:?}", args.versions);
    println!("remapped: {:?}", args.remapped);

    let manifest = match fetch_manifest().await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to fetch version manifest: {}", err);
            process::exit(1);
        }
    };

    for ele in fetch_manifest().await.unwrap().versions {
        println!("{}", ele.id)
    }
}

async fn fetch_manifest() -> Result<VersionManifest, Box<dyn Error>> {
    Ok(
        reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .await?
            .json::<VersionManifest>()
            .await?,
    )
}

#[derive(Deserialize, Debug)]
struct VersionManifest {
    versions: Vec<Manifest>,
}

#[derive(Deserialize, Debug)]
struct Manifest {
    id: String,
    url: String,
}
