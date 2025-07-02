use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rotd"));
}

#[test]
fn test_agent_info_command() {
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    cmd.args(&["agent", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rotd_cli"));
}

#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    
    cmd.current_dir(&temp_dir)
        .arg("init")
        .arg("--force")
        .assert()
        .success();
    
    // Check that .rotd directory was created
    assert!(temp_dir.path().join(".rotd").exists());
    assert!(temp_dir.path().join(".rotd/tasks.jsonl").exists());
    assert!(temp_dir.path().join(".rotd/session_state.json").exists());
}

#[test]
fn test_check_command_without_init() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    
    cmd.current_dir(&temp_dir)
        .arg("check")
        .assert()
        .failure()
        .stderr(predicate::str::contains(".rotd directory"));
}

#[test]
fn test_agent_update_task_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize first
    let mut init_cmd = Command::cargo_bin("rotd").unwrap();
    init_cmd.current_dir(&temp_dir)
        .arg("init")
        .arg("--force")
        .assert()
        .success();
    
    // Test update task with dry run
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    cmd.current_dir(&temp_dir)
        .args(&["agent", "update-task", "--dry-run"])
        .write_stdin(r#"{"id":"test","title":"Test task","status":"pending"}"#)
        .assert()
        .success();
}

#[test]
fn test_agent_update_task_invalid_json() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize first
    let mut init_cmd = Command::cargo_bin("rotd").unwrap();
    init_cmd.current_dir(&temp_dir)
        .arg("init")
        .arg("--force")
        .assert()
        .success();
    
    // Test with invalid JSON
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    cmd.current_dir(&temp_dir)
        .args(&["agent", "update-task"])
        .write_stdin("invalid json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid_json"));
}

#[test]
fn test_agent_mode_flag() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test agent mode with init
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    cmd.current_dir(&temp_dir)
        .args(&["--agent", "init", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""action":"init""#));
}

#[test]
fn test_completions_command() {
    let mut cmd = Command::cargo_bin("rotd").unwrap();
    cmd.args(&["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rotd"));
}