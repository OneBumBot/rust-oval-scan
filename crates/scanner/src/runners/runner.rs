use std::io::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CommandOutput {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub trait Runner {
    fn execute(&self, command: &str, args: &[&str]) -> Result<CommandOutput>;
    fn file_exist(&self, path: &Path) -> Result<bool>;
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn command_exits(&self, command: &str) -> Result<bool>;
    fn open_file(&self, path: &Path) -> Result<String>;
}
