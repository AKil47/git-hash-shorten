use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;

#[test]
fn test_arg_mode() {
    let mut cmd = Command::cargo_bin("ghash").unwrap();
    // We need a real git repo for this to succeed, or mock it.
    // Since we are running in the repo itself, we can try to resolve a known hash from this repo's history if possible,
    // OR we can just check that it handles the argument correctly even if it fails to resolve (due to not finding the hash).
    // However, to be robust, let's just assume we are in a git repo (which we are).
    // Let's use HEAD's hash.
    
    let output = StdCommand::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute git");
    let full_hash = String::from_utf8(output.stdout).unwrap().trim().to_string();

    cmd.arg(&full_hash)
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not()); // Should output something
}

#[test]
fn test_stdin_mode() {
    let output = StdCommand::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute git");
    let full_hash = String::from_utf8(output.stdout).unwrap().trim().to_string();

    let mut cmd = Command::cargo_bin("ghash").unwrap();
    cmd.arg("-")
        .write_stdin(full_hash)
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}
