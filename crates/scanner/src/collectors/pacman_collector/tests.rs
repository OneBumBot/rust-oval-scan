use super::*;
use crate::runners::runner::{CommandOutput, DirectoryEntry};
use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

const PACMAN_LOCAL_PATH: &str = "/var/lib/pacman/local";

struct MockRunner {
    pacman_exists: bool,
    entries: Vec<(PathBuf, bool)>,
    files: HashMap<PathBuf, String>,
}

impl MockRunner {
    fn with_packages(packages: &[(&str, &str)]) -> Self {
        let mut entries = Vec::new();
        let mut files = HashMap::new();

        for (directory, description) in packages {
            let path = Path::new(PACMAN_LOCAL_PATH).join(directory);
            entries.push((path.clone(), true));
            files.insert(path.join("desc"), (*description).to_owned());
        }

        Self {
            pacman_exists: true,
            entries,
            files,
        }
    }
}

impl Runner for MockRunner {
    async fn execute(&self, _command: &str, _args: &[&str]) -> io::Result<CommandOutput> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "execute is not used by PacmanCollector",
        ))
    }

    async fn path_exists(&self, path: &Path) -> io::Result<bool> {
        Ok(self.pacman_exists && path == Path::new(PACMAN_LOCAL_PATH))
    }

    async fn read_dir(&self, path: &Path) -> io::Result<Vec<DirectoryEntry>> {
        if path != Path::new(PACMAN_LOCAL_PATH) {
            return Err(Error::new(ErrorKind::NotFound, "directory not found"));
        }

        Ok(self
            .entries
            .iter()
            .map(|(path, is_dir)| DirectoryEntry {
                path: path.clone(),
                is_dir: *is_dir,
            })
            .collect())
    }

    async fn command_exists(&self, _command: &str) -> io::Result<bool> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "command_exists is not used by PacmanCollector",
        ))
    }

    async fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "file not found"))
    }
}

#[test]
fn name_identifies_pacman_collector() {
    let runner = MockRunner::with_packages(&[]);
    let collector = PacmanCollector::new(&runner);

    assert_eq!(collector.name(), "pacman");
}

#[tokio::test]
async fn detect_uses_pacman_database_path() {
    let detected_runner = MockRunner::with_packages(&[]);
    let missing_runner = MockRunner {
        pacman_exists: false,
        entries: Vec::new(),
        files: HashMap::new(),
    };

    assert!(
        PacmanCollector::new(&detected_runner)
            .detect()
            .await
            .expect("detection should succeed")
    );
    assert!(
        !PacmanCollector::new(&missing_runner)
            .detect()
            .await
            .expect("detection should succeed")
    );
}

#[tokio::test]
async fn collect_parses_packages_and_ignores_non_directories() {
    let mut runner = MockRunner::with_packages(&[
        (
            "alpha-1.2.3-1",
            "\
%NAME%
alpha

%VERSION%
1.2.3-1

%DESC%
Alpha package

%ARCH%
x86_64

%LICENSE%
MIT

%BUILDDATE%
100

%INSTALLDATE%
200
",
        ),
        (
            "beta-2.0-1",
            "\
%NAME%
beta

%VERSION%
2.0-1

%ARCH%
any
",
        ),
    ]);
    runner
        .entries
        .push((Path::new(PACMAN_LOCAL_PATH).join("ALPM_DB_VERSION"), false));
    let collector = PacmanCollector::new(&runner).with_concurrency(2);

    let mut packages = collector
        .collect()
        .await
        .expect("package collection should succeed");
    packages.sort_by(|left, right| left.name.cmp(&right.name));

    assert_eq!(packages.len(), 2);

    let alpha = &packages[0];
    assert_eq!(alpha.name, "alpha");
    assert_eq!(alpha.version, "1.2.3-1");
    assert_eq!(alpha.desc, "Alpha package");
    assert_eq!(alpha.arch, "x86_64");
    assert_eq!(alpha.license, "MIT");
    assert_eq!(alpha.build_date, UNIX_EPOCH + Duration::from_secs(100));
    assert_eq!(alpha.install_date, UNIX_EPOCH + Duration::from_secs(200));

    let beta = &packages[1];
    assert_eq!(beta.name, "beta");
    assert_eq!(beta.version, "2.0-1");
    assert_eq!(beta.arch, "any");
    assert_eq!(beta.desc, "");
    assert_eq!(beta.license, "");
    assert_eq!(beta.build_date, UNIX_EPOCH);
    assert_eq!(beta.install_date, UNIX_EPOCH);
}

#[tokio::test]
async fn collect_propagates_missing_description_error() {
    let runner = MockRunner {
        pacman_exists: true,
        entries: vec![(
            Path::new(PACMAN_LOCAL_PATH).join("missing-description"),
            true,
        )],
        files: HashMap::new(),
    };

    let error = PacmanCollector::new(&runner)
        .collect()
        .await
        .expect_err("missing description should fail collection");

    assert_eq!(error.kind(), ErrorKind::NotFound);
}

#[test]
fn concurrency_is_at_least_one() {
    let runner = MockRunner::with_packages(&[]);
    let collector = PacmanCollector::new(&runner).with_concurrency(0);

    assert_eq!(collector.concurrency, 1);
}
