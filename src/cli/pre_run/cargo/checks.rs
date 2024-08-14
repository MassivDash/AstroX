use std::process::Command;

use crate::cli::pre_run::utils::check_semver::check_semver;

/// Check if cargo-watch is installed
/// cargo-watch is required to spy on changes to the actix server
pub fn is_cargo_watch_installed() -> bool {
    let output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("cargo-watch")
}

/// Check if commitlint-rs is installed
/// commitlint-rs is required to lint the commit messages in development env
pub fn is_commitlint_rs_installed() -> bool {
    let output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("commitlint-rs")
}

/// Check if llvm-cov is installed
/// llvm-cov is required to generate the coverage report
/// llvm-cov is used in the coverage command

pub fn is_llvm_cov_installed() -> bool {
    let output = Command::new("cargo")
        .arg("llvm-cov")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("cargo-llvm-cov")
}

pub const REQUIRED_VERSION: &str = "1.74.0";

pub fn is_rustc_higher_than_required() -> bool {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut version = output_str.trim();

    // output is like "rustc 1.74.0 (d1dfba2c9 2021-07-26)"
    // we need to extract the version
    version = version.split_whitespace().collect::<Vec<&str>>()[1];

    check_semver(version, REQUIRED_VERSION)
}
