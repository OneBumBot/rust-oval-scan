pub mod host_collector;
pub mod package_collector;
pub mod pacman_collector;

use crate::{
    collectors::{package_collector::PackageCollector, pacman_collector::PacmanCollector},
    runners::runner::Runner,
};
use futures::io;
use tracing;

#[tracing::instrument(name = "collectors.select_collector", skip(runner), err)]
pub async fn select_package_collectors<'a, R: Runner>(
    runner: &'a R,
) -> io::Result<Vec<Box<dyn PackageCollector + 'a>>> {
    let mut res: Vec<Box<dyn PackageCollector + 'a>> = Vec::new();

    let collectors = vec![Box::new(PacmanCollector::new(runner))];

    for collector in collectors {
        if collector.detect().await? {
            tracing::info!(collector = collector.name(), "Package collector detected");
            res.push(collector);
        }
    }
    tracing::trace!(
        collecotrs.count = res.len(),
        "Package collectors detecion complete"
    );
    Ok(res)
}
