use std::io;

use oval_scanner::collectors::host_collector::HostCollector;

fn main() -> io::Result<()> {
    let host = HostCollector::collect()?;

    println!("Host: {host:#?}");
    Ok(())
}
