use chrono::{DateTime, Local};
use std::io;
use std::time::{Instant, SystemTime};

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

#[tokio::main]
async fn main() -> io::Result<()> {
    let runner = LocalRunner;

    let host = HostCollector::collect(&runner).await?;

    println!("Host: {host:#?}");

    let collector = PacmanCollector::new(&runner).with_concurrency(16);

    let name = collector.name();
    let detect = collector.detect().await?;

    if detect {
        println!("{} was detected", name);
        println!("Parse all packages in system");
        let started_at = SystemTime::now();
        let timer = Instant::now();
        let packages = collector.collect().await?;
        let elapsed = timer.elapsed();
        let finished_at = SystemTime::now();
        println!("Show first 10 packages:");
        for package in packages.iter().take(40) {
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
