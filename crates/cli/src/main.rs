use std::io;

use oval_core::hosts::Host;

fn main() -> io::Result<()> {
    let host = Host::collect()?;

    println!("Host: {host:#?}");
    Ok(())
}
