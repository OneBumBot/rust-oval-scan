use chrono::{DateTime, Local};
use oval_scanner::runners::local_runner::LocalRunner;
use oval_scanner::scan::scan_host;
use std::fs::File;
use std::io::Write;
use std::io::{self, IsTerminal};
use std::time::{Instant, SystemTime};
use tracing_subscriber::EnvFilter;

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

fn dump_inventory_to_file(host: &oval_core::hosts::Host) -> io::Result<()> {
    let mut file = File::create("inventory.json")?;
    let json = serde_json::to_string_pretty(host)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    init_tracing();
    run().await
}

#[tracing::instrument(name = "scan", skip_all, err)]
async fn run() -> io::Result<()> {
    let started_at = SystemTime::now();
    let timer = Instant::now();

    let runner = LocalRunner;

    let host = scan_host(&runner).await?;

    let elapsed = timer.elapsed();
    let finished_at = SystemTime::now();
    println!("Host: {}", serde_json::to_string(&host)?);
    println!(
        "Collection started: {}\n\
            Collection finished: {}\n\
            Collection duration: {:.2?}\n\
            Packages collected: {}",
        format_time(started_at),
        format_time(finished_at),
        elapsed,
        host.packages.len(),
    );

    dump_inventory_to_file(&host);

    Ok(())
}
