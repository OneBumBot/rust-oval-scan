use chrono::{DateTime, Local};
use oval_scanner::collectors;
use std::io::{self, IsTerminal};
use std::time::{Instant, SystemTime};
use tracing_subscriber::EnvFilter;

use oval_scanner::{
    collectors::{
        host_collector::HostCollector, package_collector::PackageCollector,
        pacman_collector::PacmanCollector,
    },
    runners::local_runner::LocalRunner,
};

fn format_time(time: SystemTime) -> String {
    DateTime::<Local>::from(time)
        .format("%Y-%m-%d %H:%M:%S %:z")
        .to_string()
}

fn init_tracing() {
    let ansi = std::io::stderr().is_terminal();

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(ansi)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .compact()
        .init()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    init_tracing();
    run().await
}

#[tracing::instrument(name = "scan", skip_all, err)]
async fn run() -> io::Result<()> {
    let runner = LocalRunner;

    let host = HostCollector::collect(&runner).await?;

    println!("Host: {host:#?}");

    let collectors = collectors::select_package_collectors(&runner).await?;

    if collectors.is_empty() {
        tracing::warn!("No supported collectors detected");
    }

    for collector in collectors {
        let started_at = SystemTime::now();
        let timer = Instant::now();
        let packages = collector.collect().await?;
        let elapsed = timer.elapsed();
        let finished_at = SystemTime::now();
        let packages_to_take = 40;
        println!("Show first {} packages:", packages_to_take);
        for package in packages.iter().take(packages_to_take) {
            println!(
                "{} {} (installed: {}, built: {})",
                package.name,
                package.version,
                format_time(package.install_date),
                format_time(package.build_date),
            );
        }

        println!(
            "Collection started: {}\n\
            Collection finished: {}\n\
            Collection duration: {:.2?}\n\
            Packages collected: {}",
            format_time(started_at),
            format_time(finished_at),
            elapsed,
            packages.len(),
        );
    }

    Ok(())
}
