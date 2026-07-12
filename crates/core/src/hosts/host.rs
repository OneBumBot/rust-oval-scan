use super::os;

#[derive(Debug, Default)]
pub struct Host {
    pub hostname: String,
    pub kernel: String,
    pub arch: String,
    pub os_info: os::OsInfo,
}
