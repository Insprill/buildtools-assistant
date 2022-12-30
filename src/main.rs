use std::{
    cmp::max,
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use futures::future;
use itertools::Itertools;
use log::{info, LevelFilter};

use clap::{command, Parser};
use platform_dirs::AppDirs;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use sysinfo::{CpuRefreshKind, RefreshKind, System, SystemExt};
use tokio::runtime::Builder;

pub mod adoptium;
pub mod mojang;
pub mod os;
pub mod spigot;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What versions to run BuildTools for
    versions: Vec<String>,

    /// How many BuildTools instances to run simultaneously. Defaults to 1/4 the CPU's core count.
    #[arg(short, long)]
    workers: Option<usize>,

    /// How much memory to give each BuildTools instance, in MB
    #[arg(short, long, default_value = "512")]
    bt_mem: Option<usize>,

    /// Whether BuildTools' full output should be printed or not
    #[arg(short, long)]
    verbose: bool,

    /// The path where the built Spigot/CraftBukkit jars will be placed
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])?;

    let mut sys = System::new_with_specifics(
        RefreshKind::default()
            .with_memory()
            .with_cpu(CpuRefreshKind::new()),
    );
    sys.refresh_cpu();
    sys.refresh_memory();

    let args = Args::parse();
    let worker_count = args.workers.unwrap_or_else(|| max(1, sys.cpus().len() / 4));

    let runtime = Builder::new_multi_thread()
        .worker_threads(worker_count)
        .enable_time()
        .enable_io()
        .build()?;

    let bt_mem = args.bt_mem.unwrap_or(512);

    if bt_mem < 512 {
        panic!("BuildTools must have at least 512MB of memory per-instance!");
    }
    if ((bt_mem * worker_count) * 1_000_000) as u64 > sys.available_memory() {
        panic!("You don't have enough memory to run {} BuildTools instances with {}MB of memory! Please lower the worker count or memory available to each instance.", worker_count, bt_mem);
    }

    runtime.block_on(run(args.versions, bt_mem, args.output_dir, args.verbose))
}

async fn run(
    versions: Vec<String>,
    bt_mem: usize,
    output_dir: Option<PathBuf>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let manifests = mojang::map_version_manifests(&versions).await?;

    if let Some(invalid_ver) = spigot::versions_exist(&versions).await? {
        panic!("BuildTools doesn't support version {}!", invalid_ver);
    }

    let packages = mojang::fetch_packages(manifests.clone()).await?;

    let java_versions: Vec<u8> = packages
        .iter()
        .map(|p| &p.java_version)
        .map(|v| v.major_version)
        .unique()
        .collect();

    let java_releases = adoptium::get_releases().await?;

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

    adoptium::try_download_versions(&java_releases, java_versions, &java_dir).await?;

    let buildtools_jar_data = spigot::download_buildtools().await?;

    let bt_file_dir = cache_dir.join("buildtools.jar");

    let mut bt_file = File::create(&bt_file_dir)?;
    bt_file.set_len(0)?;
    bt_file.write_all(&buildtools_jar_data)?;

    let rand: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();
    let bt_tmp_dir = env::temp_dir().join(format!("{}-{}", env!("CARGO_PKG_NAME"), rand));
    info!("Created temp directory at {}", bt_tmp_dir.to_string_lossy());

    let mut handles = Vec::with_capacity(packages.len());
    for package in packages {
        let bt_dir = bt_tmp_dir.join(&package.id);
        fs::create_dir_all(&bt_dir)?;

        let java_dir = java_dir.clone();
        let bt_file_dir = bt_file_dir.clone();
        let output_dir = output_dir.as_ref().unwrap_or(&bt_dir).clone();

        let install_dir =
            adoptium::get_java_install(package.java_version.major_version, &java_dir).await?;

        handles.push(tokio::spawn(async move {
            info!("Running BuildTools for {}", package.id);
            Command::new(install_dir.to_string_lossy().to_string())
                .arg(format!("-Xmx{}m", bt_mem))
                .arg("-jar")
                .arg(&bt_file_dir.to_string_lossy().to_string())
                .arg("--rev")
                .arg(package.id)
                .arg("--output-dir")
                .arg(output_dir.to_string_lossy().to_string())
                .arg("--remapped")
                .current_dir(&bt_dir)
                .stderr(Stdio::inherit())
                .stdout(if verbose {
                    Stdio::inherit()
                } else {
                    Stdio::null()
                })
                .output()
                .expect("Failed to run BuildTools");
        }));
    }
    future::join_all(handles).await;

    info!("Finished running BuildTools! Removing temp directory.");
    fs::remove_dir_all(bt_tmp_dir)?;

    Ok(())
}
