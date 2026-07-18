use oval_core::hosts::Host;
use oval_core::hosts::os;
use std::{fs, io, process::Command};

pub struct HostCollector {}

fn get_hostname() -> io::Result<String> {
    // `/etc/hostname` is provided by Linux systems, while the `hostname`
    // executable is an optional userspace utility and may not be installed.
    Ok(fs::read_to_string("/etc/hostname")?.trim().to_owned())
}

fn get_kernel() -> io::Result<String> {
    let output = Command::new("uname").arg("-r").output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn get_arch() -> io::Result<String> {
    let output = Command::new("uname").arg("-m").output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

impl HostCollector {
    pub fn collect() -> io::Result<Host> {
        Ok(Host {
            hostname: get_hostname()?,
            kernel: get_kernel()?,
            arch: get_arch()?,
            os_info: os::parse_os_release()?,
        })
    }
}
