use crate::{collectors::package_collector::PackageCollector, runners::runner::Runner};
use oval_core::packages;
use oval_core::packages::package::Package;
use std::time::{Duration, UNIX_EPOCH};
use std::{collections::HashMap, fs, io, path::Path};

pub struct PacmanCollector<R> {
    runner: R,
}

impl<R: Runner> PacmanCollector<R> {
    pub fn new(runner: R) -> Self {
        Self { runner }
    }

    fn parse_alpm_file(&self, path: &Path) -> io::Result<HashMap<String, Vec<String>>> {
        let content = self.runner.open_file(path)?;
        Ok(self.parse_alpm_content(&content))
    }

    fn parse_alpm_content(&self, content: &str) -> HashMap<String, Vec<String>> {
        let mut desc: HashMap<String, Vec<String>> = HashMap::new();

        let mut current_field: Option<String> = None;
        let mut values: Vec<String> = Vec::new();

        for line in content.lines() {
            match line {
                "" => (),

                val if val.starts_with("%") && val.ends_with("%") => {
                    if let Some(field) = current_field.take() {
                        desc.insert(field, std::mem::take(&mut values));
                    }
                    current_field = Some(val.trim_matches('%').to_owned());
                }

                val => {
                    if current_field.is_some() {
                        values.push(val.to_owned());
                    }
                }
            }
        }

        if let Some(field) = current_field {
            desc.insert(field, values);
        }

        desc
    }

    fn parse_alpm_to_package(&self, hashmap: &HashMap<String, Vec<String>>) -> Package {
        Package {
            arch: hashmap
                .get("ARCH")
                .and_then(|values| values.first())
                .cloned()
                .unwrap_or_else(|| "".to_string()),
            name: hashmap
                .get("NAME")
                .and_then(|values| values.first())
                .cloned()
                .unwrap_or_else(|| "".to_string()),
            version: hashmap
                .get("VERSION")
                .and_then(|values| values.first())
                .cloned()
                .unwrap_or_else(|| "".to_string()),
            desc: hashmap
                .get("DESC")
                .and_then(|values| values.first())
                .cloned()
                .unwrap_or_else(|| "".to_string()),
            license: hashmap
                .get("LICENSE")
                .and_then(|values| values.first())
                .cloned()
                .unwrap_or_else(|| "".to_string()),
            build_date: hashmap
                .get("BUILDDATE")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<u64>().ok())
                .map(|seconds| UNIX_EPOCH + Duration::from_secs(seconds))
                .unwrap_or(UNIX_EPOCH),
            install_date: hashmap
                .get("INSTALLDATE")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<u64>().ok())
                .map(|seconds| UNIX_EPOCH + Duration::from_secs(seconds))
                .unwrap_or(UNIX_EPOCH),
        }
    }
}

impl<R: Runner> PackageCollector for PacmanCollector<R> {
    fn name(&self) -> &'static str {
        "pacman"
    }

    fn detect(&self) -> std::io::Result<bool> {
        self.runner.file_exist(Path::new("/var/lib/pacman/local/"))
    }

    fn collect(&self) -> std::io::Result<Vec<Package>> {
        let pacman_dir_path = "/var/lib/pacman/local";
        let mut packages: Vec<Package> = Vec::new();
        for entry in self.runner.read_dir(Path::new(pacman_dir_path))? {
            if !entry.is_file() {
                let content = self.parse_alpm_file(entry.join(Path::new("desc")).as_path())?;
                let package = self.parse_alpm_to_package(&content);
                packages.push(package);
            }
        }

        Ok(packages)
    }
}
