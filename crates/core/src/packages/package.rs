#[derive(Debug, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub desc: Option<String>,
    pub arch: Option<String>,
    pub license: String,
    pub build_date: String,
    pub install_date: String,
}
