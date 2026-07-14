use crate::{collectors::package_collector::PackageCollector, runners::runner::Runner};
use oval_core::packages::package::Package;
use std::{collections::HashMap, io, path::Path};

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
                "" => println!(),

                val if val.starts_with("%") && val.ends_with("%") => {
                    if let Some(field) = current_field.take() {
                        desc.insert(field, std::mem::take(&mut values));
                    }
                    current_field = Some(val.trim_matches('%').to_owned());
                    println!("{}", val.trim_matches('%').to_owned());
                }

                val => {
                    if current_field.is_some() {
                        values.push(val.to_owned());
                        println!("{}", val)
                    }
                }
            }
        }

        if let Some(field) = current_field {
            desc.insert(field, values);
        }

        desc
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
        let res =
            self.parse_alpm_file(Path::new("/var/lib/pacman/local/zellij-0.44.3-1.1/desc"))?;

        println!("{:#?}", res);
        todo!()
    }
}
