use std::{
    env,
    fs::{self, File},
    io::Write,
    process::{Command, Stdio},
};

use futures::future;
use itertools::Itertools;
use log::LevelFilter;

use clap::{command, Parser};
use platform_dirs::AppDirs;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

pub mod adoptium;
pub mod mojang;
pub mod spigot;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What versions to run BuildTools for
    versions: Vec<String>,
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

    let manifests = mojang::map_version_manifests(args.versions)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to fetch version manifest: {:?}", err);
        });

    let packages = mojang::fetch_packages(manifests.clone())
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to fetch version manifest: {:?}", err);
        });

    let java_versions: Vec<u8> = packages
        .iter()
        .map(|p| &p.java_version)
        .map(|v| v.major_version)
        .unique()
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
    let cache_dir = app_dirs.cache_dir;

    let java_dir = cache_dir.join("java");

    adoptium::try_download_versions(java_versions, &java_dir)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to download Java: {:?}", err);
        });

    let buildtools_jar_data = spigot::download_buildtools().await.unwrap_or_else(|err| {
        panic!("Failed to download buildtools: {:?}", err);
    });

    let bt_file_dir = cache_dir.join("buildtools.jar");
    let mut bt_file = File::create(&bt_file_dir).unwrap_or_else(|err| {
        panic!("Failed to create buildtools file: {:?}", err);
    });
    bt_file.set_len(0).unwrap_or_else(|err| {
        panic!("Failed to remove old BuildTools jar: {:?}", err);
    });
    bt_file
        .write_all(&buildtools_jar_data)
        .unwrap_or_else(|err| {
            panic!("Failed to write to BuildTools jar: {:?}", err);
        });

    let bt_tmp_dir = env::temp_dir().join("buildtools");

    let mut handles = Vec::with_capacity(packages.len());
    for package in packages {
        let bt_dir = bt_tmp_dir.join(&package.id);
        fs::create_dir_all(&bt_dir).unwrap_or_else(|err| {
            panic!("Failed to create BuildTools dir: {:?}", err);
        });
        let java_dir = java_dir.clone();
        let bt_file_dir = bt_file_dir.clone();
        handles.push(tokio::spawn(async move {
            let install_dir =
                adoptium::get_java_install(package.java_version.major_version, &java_dir)
                    .await
                    .unwrap_or_else(|err| {
                        panic!(
                            "Failed to find Java {} install: {:?}",
                            package.java_version.major_version, err
                        );
                    });
            Command::new(install_dir.to_string_lossy().to_string())
                .arg("-jar")
                .arg(&bt_file_dir.to_string_lossy().to_string())
                .arg("--rev")
                .arg(package.id)
                .arg("--remapped")
                .current_dir(&bt_dir)
                .stdout(Stdio::inherit())
                .output()
                .expect("Failed to run BuildTools");
        }));
    }
    future::join_all(handles).await;

    fs::remove_dir_all(bt_tmp_dir).expect("Failed to remove temp directory!");
}
