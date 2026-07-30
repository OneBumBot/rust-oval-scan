use futures::future::BoxFuture;
use oval_core::packages::package::Package;
use std::future::Future;
use std::io;

pub trait PackageCollector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self) -> impl Future<Output = io::Result<bool>> + Send;
    fn collect(&self) -> impl Future<Output = io::Result<Vec<Package>>> + Send;
}
