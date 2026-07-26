use crate::runners::runner::{CommandOutput, DirectoryEntry, Runner};
use std::{io::Result, path::Path};

pub struct LocalRunner;

impl Runner for LocalRunner {
    async fn execute(&self, command: &str, args: &[&str]) -> Result<CommandOutput> {
        let mut command = tokio::process::Command::new(command);
        command.args(args).kill_on_drop(true);

        let output = command.output().await?;

        Ok(CommandOutput {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    async fn path_exists(&self, path: &Path) -> Result<bool> {
        tokio::fs::try_exists(path).await
    }

    async fn command_exists(&self, command: &str) -> Result<bool> {
        let res = tokio::process::Command::new("sh")
            .args(["-c", "command -v -- \"$1\"", "sh", command])
            .kill_on_drop(true)
            .output()
            .await?;

        Ok(res.status.success())
    }

    async fn read_to_string(&self, path: &Path) -> Result<String> {
        tokio::fs::read_to_string(path).await
    }

    async fn read_dir(&self, path: &Path) -> Result<Vec<DirectoryEntry>> {
        let mut dir = tokio::fs::read_dir(path).await?;
        let mut entries: Vec<DirectoryEntry> = Vec::new();
        while let Some(entry) = dir.next_entry().await? {
            entries.push(DirectoryEntry {
                path: entry.path(),
                is_dir: entry.file_type().await?.is_dir(),
            });
        }
        Ok(entries)
    }
}

#[cfg(test)]
mod tests;
