use crate::runners::runner::{CommandOutput, Runner};

use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub struct LocalRunner;

impl Runner for LocalRunner {
    fn execute(
        &self,
        command: &str,
        args: &[&str],
    ) -> io::Result<crate::runners::runner::CommandOutput> {
        let output = std::process::Command::new(command).args(args).output()?;
        Ok(CommandOutput {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    fn file_exist(&self, path: &Path) -> io::Result<bool> {
        // match fs::metadata(path) {
        //     Ok(_) => Ok(true),
        //     Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        //     Err(error) => Err(error),
        // }
        Ok(path.try_exists()?)
    }

    fn command_exits(&self, command: &str) -> io::Result<bool> {
        let res = std::process::Command::new("sh")
            .args(["-c", "command -v -- \"$1\"", "sh", command])
            .output()?;

        Ok(res.status.success())
    }

    fn open_file(&self, path: &Path) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
