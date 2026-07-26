use std::{
    future::Future,
    io::Result,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct CommandOutput {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug)]
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub is_dir: bool,
}

pub trait Runner: Send + Sync {
    fn execute(
        &self,
        command: &str,
        args: &[&str],
    ) -> impl std::future::Future<Output = Result<CommandOutput>> + Send;
    fn path_exists(&self, path: &Path) -> impl std::future::Future<Output = Result<bool>> + Send;
    fn read_dir(
        &self,
        path: &Path,
    ) -> impl std::future::Future<Output = Result<Vec<DirectoryEntry>>> + Send;
    fn command_exists(&self, command: &str) -> impl Future<Output = Result<bool>> + Send;
    fn read_to_string(&self, path: &Path) -> impl Future<Output = Result<String>> + Send;
}
