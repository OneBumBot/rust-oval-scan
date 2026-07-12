use std::io::Result;
use std::process::Output;

trait Runner {
    fn execute(&self, command: &str, args: &str) -> Result<Output>;
    fn is_file_exist(&self, path: &str) -> Result<bool>;
    fn open_file(&self, path: &str) -> String;
}
