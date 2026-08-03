use futures::future::BoxFuture;
use oval_core::packages::package::Package;
use std::io;

pub trait PackageCollector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self) -> BoxFuture<'_, io::Result<bool>>;
    fn collect(&self) -> BoxFuture<'_, io::Result<Vec<Package>>>;
}
