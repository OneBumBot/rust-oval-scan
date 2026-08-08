use crate::{
    collectors::{host_collector::HostCollector, select_package_collectors},
    runners::runner::Runner,
};
use oval_core::hosts::Host;
use std::io;

pub async fn scan_host<R: Runner>(runner: &R) -> io::Result<Host> {
    let mut host = HostCollector::collect(runner).await?;

    for collector in select_package_collectors(runner).await? {
        host.packages.extend(collector.collect().await?)
    }

    Ok(host)
}
