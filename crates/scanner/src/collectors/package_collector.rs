use crate::runners::runner::Runner;
use oval_core::packages::package::Package;
use std::io;

pub trait PackageCollector {
    fn name(&self) -> &'static str;
    fn detect(&self) -> io::Result<bool>;
    fn collect(&self) -> io::Result<Vec<Package>>;
}
