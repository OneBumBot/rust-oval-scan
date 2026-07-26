use crate::{collectors::package_collector::PackageCollector, runners::runner::Runner};
use futures::{StreamExt, TryStreamExt, stream};
use oval_core::packages::package::Package;
use std::{
    collections::HashMap,
    io,
    path::Path,
    time::{Duration, UNIX_EPOCH},
};

pub struct PacmanCollector<'a, R> {
    runner: &'a R,
    concurrency: usize,
}

impl<'a, R: Runner> PacmanCollector<'a, R> {
    pub fn new(runner: &'a R) -> Self {
        Self {
            runner,
            concurrency: 16,
        }
    }

    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    async fn parse_alpm_file(&self, path: &Path) -> io::Result<HashMap<String, Vec<String>>> {
        let content = self.runner.read_to_string(path).await?;
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

impl<'a, R: Runner> PackageCollector for PacmanCollector<'a, R> {
    fn name(&self) -> &'static str {
        "pacman"
    }

    async fn detect(&self) -> std::io::Result<bool> {
        self.runner
            .path_exists(Path::new("/var/lib/pacman/local/"))
            .await
    }

    async fn collect(&self) -> std::io::Result<Vec<Package>> {
        let entries = self
            .runner
            .read_dir(Path::new("/var/lib/pacman/local/"))
            .await?;

        let package_dirs = entries.into_iter().filter(|entry| entry.is_dir);

        let packages = stream::iter(package_dirs)
            .map(|entry| async move {
                let path = entry.path.join("desc");
                let parsed = self.parse_alpm_file(&path).await?;
                Ok::<_, io::Error>(self.parse_alpm_to_package(&parsed))
            })
            .buffer_unordered(self.concurrency)
            .try_collect()
            .await?;

        Ok(packages)
    }
}

#[cfg(test)]
mod tests;
