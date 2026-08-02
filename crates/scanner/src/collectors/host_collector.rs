use crate::runners::runner::Runner;
use oval_core::hosts::Host;
use oval_core::hosts::os;
use std::path::Path;
use std::{fs, io, process::Command};

pub struct HostCollector {}

impl HostCollector {
    #[tracing::instrument(name = "host.collect", skip_all, err)]
    pub async fn collect<R: Runner>(runner: &R) -> io::Result<Host> {
        let (hostname, kernel, arch, os_release) = tokio::try_join!(
            runner.read_to_string(Path::new("/etc/hostname")),
            runner.execute("uname", &["-r"]),
            runner.execute("uname", &["-m"]),
            runner.read_to_string(Path::new("/etc/os-release")),
        )?;

        Ok(Host {
            hostname: hostname.trim().to_owned(),
            kernel: kernel.stdout.trim().to_owned(),
            arch: arch.stdout.trim().to_owned(),
            os_info: os::parse_os_release(&os_release)?,
        })
    }
}
