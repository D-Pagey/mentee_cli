use assert_cmd::Command;
use mentees::mentee_service::MenteeService;
use predicates::prelude::predicate;
use std::{fs, path::Path};

#[test]
fn test_empty_mentees() {
    let database_url = "test_database.db";
    setup_test_database(database_url);

    Command::cargo_bin("mentees")
        .unwrap()
        .args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Name")); // check table renders header

    fs::remove_file(database_url).unwrap();
}

// #[test]
// fn test_add_mentee() {
//     let database_url = "test_database.db";
//     setup_test_database(database_url);
//
//     Command::cargo_bin("mentees")
//         .unwrap()
//         .args(&["list"])
//         .assert()
//         .success()
//         .stdout(predicate::str::contains("Name"));
//
//     Command::cargo_bin("mentees")
//         .unwrap()
//         .args(&["add"])
//         .write_stdin("Dan Page/n")
//         .assert()
//         .success()
//         .stdout(predicate::str::contains("What is their name?"));
//
//     fs::remove_file(database_url).unwrap();
// }

fn setup_test_database(database_url: &str) {
    // Ensure no leftover database from previous tests
    if Path::new(database_url).exists() {
        fs::remove_file(database_url).unwrap();
    }

    let _ = MenteeService::new(database_url);
}
