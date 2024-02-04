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

// check if node is installed
// check if the node version is above 18.14.1
pub fn is_node_installed() -> bool {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut version = output_str.trim();

    // remove the v from v20.10.0
    if version.starts_with('v') {
        version = &version[1..];
    }

    let version_split: Vec<&str> = version.split('.').collect();
    if version_split[0].parse::<i32>().unwrap() > 18 {
        return true;
    };
    if version_split[0].parse::<i32>().unwrap() == 18
        && version_split[1].parse::<i32>().unwrap() > 14
    {
        return true;
    };
    if version_split[0].parse::<i32>().unwrap() == 18
        && version_split[1].parse::<i32>().unwrap() == 14
        && version_split[2].parse::<i32>().unwrap() >= 1
    {
        return true;
    };
    false
}

pub fn is_frontend_project_installed() -> bool {
    let project = std::path::Path::new("./src/frontend/package.json").exists();
    let installed = std::path::Path::new("./src/frontend/node_modules").exists();
    project && installed
}
