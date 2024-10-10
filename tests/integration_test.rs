use assert_cmd::Command;
use mentees::mentee_service::MenteeService;
use predicates::prelude::predicate;
use std::{env, fs, path::Path};

#[test]
fn test_empty_mentees() {
    setup_test_database();

    Command::cargo_bin("mentees")
        .unwrap()
        .args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Name")); // check table renders header
}

fn setup_test_database() {
    let mut db_path = env::temp_dir();
    db_path.push("test_mentees.db");

    // Ensure no leftover database from previous tests
    if Path::new(&db_path).exists() {
        fs::remove_file(&db_path).unwrap();
    }

    let _ = MenteeService::new(true);
}
