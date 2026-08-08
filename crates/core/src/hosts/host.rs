use super::os;
use crate::packages::package::Package;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub kernel: String,
    pub arch: String,
    pub os_info: os::OsInfo,
    pub packages: Vec<Package>,
}
