use oval_core::packages::package::Package;
use std::io;

pub trait PackageCollector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self) -> impl std::future::Future<Output = io::Result<bool>> + Send;
    fn collect(&self) -> impl std::future::Future<Output = io::Result<Vec<Package>>> + Send;
}
