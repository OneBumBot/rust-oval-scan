use std::io;
use std::path::Path;

use oval_scanner::collectors::host_collector::HostCollector;
use oval_scanner::collectors::package_collector::PackageCollector;
use oval_scanner::collectors::pacman_collector::PacmanCollector;
use oval_scanner::runners::local_runner::LocalRunner;
use oval_scanner::runners::runner::{self, Runner};

fn main() -> io::Result<()> {
    let host = HostCollector::collect()?;

    println!("Host: {host:#?}");

    let runner = LocalRunner {};

    let collector = PacmanCollector::new(runner);

    let name = collector.name();
    let detect = collector.detect()?;

    if detect {
        println!("{} was detected", name);
        collector.collect()?;
    }

    Ok(())
}
