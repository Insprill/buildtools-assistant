use log::LevelFilter;

use clap::{arg, command, Parser};
use platform_dirs::AppDirs;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

pub mod adoptium;
pub mod mojang;

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

    let manifests = mojang::map_version_manifests(args.versions)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to fetch version manifest: {:?}", err);
        });

    let packages = mojang::fetch_packages(manifests)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to fetch version manifest: {:?}", err);
        });

    let java_versions: Vec<u8> = packages
        .into_iter()
        .map(|p| p.javaVersion)
        .map(|v| v.majorVersion)
        .collect();

    let java_releases = adoptium::get_releases().await.unwrap_or_else(|err| {
        panic!("Failed to fetch available Java versions: {:?}", err);
    });

    if let Some(unavail) = java_versions
        .iter()
        .find(|v| !java_releases.available_releases.contains(v))
    {
        panic!("Failed to find Java version: {:?}", unavail);
    }

    let app_dirs =
        AppDirs::new(Some(env!("CARGO_PKG_NAME")), false).expect("Failed to find app dir");
    let java_dir = app_dirs.cache_dir.join("java");

    adoptium::try_download_versions(java_versions, &java_dir).await;
}
