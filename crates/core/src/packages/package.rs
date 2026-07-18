use std::time::SystemTime;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub desc: String,
    pub arch: String,
    pub license: String,
    pub build_date: SystemTime,
    pub install_date: SystemTime,
}
