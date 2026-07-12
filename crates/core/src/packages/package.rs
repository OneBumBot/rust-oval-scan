#[derive(Debug, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub license: String,
    pub build_date: String,
    pub install_date: String,
}
