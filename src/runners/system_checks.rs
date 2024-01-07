use std::process::Command;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";


pub fn is_cargo_watch_installed() -> bool {
    let output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("cargo-watch")
}

pub fn is_node_installed() -> bool {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("v")
}