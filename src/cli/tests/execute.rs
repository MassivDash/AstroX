use std::process::Command;

use crate::cli::{
    pre_run::npm::checks::NPM,
    utils::terminal::{step, success},
};

pub fn execute_backend_tests() {
    step("Executing backend tests");

    let cargo_test = Command::new("cargo")
        .current_dir("./src/backend")
        .arg("test")
        .spawn()
        .expect("Failed to execute backend tests")
        .wait()
        .expect("Failed to execute backend tests");

    match cargo_test.success() {
        true => step("Backend tests executed successfully"),
        false => panic!("Failed to execute backend tests"),
    }
}

pub fn execute_cli_coverage() {
    step("Executing CLI coverage");

    let cargo_coverage = Command::new("cargo")
        .current_dir("./")
        .arg("llvm-cov")
        .arg("--all-features")
        .arg("--workspace")
        .spawn()
        .expect("Failed to execute CLI coverage")
        .wait()
        .expect("Failed to execute CLI coverage");

    match cargo_coverage.success() {
        true => success("CLI coverage executed successfully"),
        false => panic!("Failed to execute CLI coverage"),
    }
}

pub fn execute_backend_coverage() {
    step("Executing CLI coverage");

    let cargo_coverage = Command::new("cargo")
        .current_dir("./src/backend/")
        .arg("llvm-cov")
        .arg("--all-features")
        .arg("--workspace")
        .spawn()
        .expect("Failed to execute CLI coverage")
        .wait()
        .expect("Failed to execute CLI coverage");

    match cargo_coverage.success() {
        true => success("CLI coverage executed successfully"),
        false => panic!("Failed to execute CLI coverage"),
    }
}

pub fn execute_frontend_tests() {
    step("Executing frontend tests");

    let npm_test = Command::new(NPM)
        .arg("run")
        .arg("test")
        .current_dir("./src/frontend")
        .spawn()
        .expect("Failed to execute frontend tests")
        .wait()
        .expect("Failed to execute frontend tests");

    match npm_test.success() {
        true => step("Frontend tests executed successfully"),
        false => panic!("Failed to execute frontend tests"),
    }
}

pub fn execute_tests() {
    execute_backend_tests();
    execute_frontend_tests();
}

pub fn execute_coverage() {
    execute_cli_coverage();
    execute_backend_coverage();
}
