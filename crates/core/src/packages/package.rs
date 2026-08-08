use serde::{Deserialize, Serialize};
use std::time::SystemTime;
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub desc: String,
    pub arch: String,
    pub license: String,
    pub build_date: SystemTime,
    pub install_date: SystemTime,
}
