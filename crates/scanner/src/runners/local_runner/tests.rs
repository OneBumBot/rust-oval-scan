use super::*;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn execute_captures_exit_code_stdout_and_stderr() {
    let runner = LocalRunner;

    let output = runner
        .execute(
            "sh",
            &[
                "-c",
                "printf 'standard output'; printf 'standard error' >&2; exit 7",
            ],
        )
        .await
        .expect("shell command should run");

    assert_eq!(output.exit_code, Some(7));
    assert_eq!(output.stdout, "standard output");
    assert_eq!(output.stderr, "standard error");
}

#[tokio::test]
async fn path_exists_distinguishes_existing_and_missing_paths() {
    let runner = LocalRunner;
    let temp_dir = tempdir().expect("temporary directory should be created");
    let existing_path = temp_dir.path().join("existing");
    let missing_path = temp_dir.path().join("missing");
    fs::write(&existing_path, "content").expect("fixture file should be written");

    assert!(
        runner
            .path_exists(&existing_path)
            .await
            .expect("existing path should be checked")
    );
    assert!(
        !runner
            .path_exists(&missing_path)
            .await
            .expect("missing path should be checked")
    );
}

#[tokio::test]
async fn read_to_string_returns_file_contents() {
    let runner = LocalRunner;
    let temp_dir = tempdir().expect("temporary directory should be created");
    let path = temp_dir.path().join("data");
    fs::write(&path, "hello\nworld").expect("fixture file should be written");

    let content = runner
        .read_to_string(&path)
        .await
        .expect("fixture file should be read");

    assert_eq!(content, "hello\nworld");
}

#[tokio::test]
async fn read_dir_returns_paths_and_file_types() {
    let runner = LocalRunner;
    let temp_dir = tempdir().expect("temporary directory should be created");
    let file_path = temp_dir.path().join("file");
    let directory_path = temp_dir.path().join("directory");
    fs::write(&file_path, "content").expect("fixture file should be written");
    fs::create_dir(&directory_path).expect("fixture directory should be created");

    let entries = runner
        .read_dir(temp_dir.path())
        .await
        .expect("temporary directory should be read");

    assert_eq!(entries.len(), 2);
    assert!(
        entries
            .iter()
            .any(|entry| entry.path == file_path && !entry.is_dir)
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry.path == directory_path && entry.is_dir)
    );
}

#[tokio::test]
async fn command_exists_reports_found_and_missing_commands() {
    let runner = LocalRunner;

    assert!(
        runner
            .command_exists("sh")
            .await
            .expect("existing command should be checked")
    );
    assert!(
        !runner
            .command_exists("oval-command-that-does-not-exist")
            .await
            .expect("missing command should be checked")
    );
}
