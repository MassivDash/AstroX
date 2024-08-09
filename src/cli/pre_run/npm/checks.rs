use std::process::Command;

use crate::cli::pre_run::utils::check_semver::check_semver;
/// The name of the npm executable.
/// On windows it is npm.cmd
/// On linux and mac it is npm

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &str = "npm";

const REQUIRED_VERSION: &str = "18.14.1";

/// check if node is installed
/// check if the node version is above 18.14.1
/// versions below will panic on astro commands
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

    check_semver(version, REQUIRED_VERSION)
}

/// Check if the frontend project is installed
/// The frontend project is required to run the astro commands

pub fn is_frontend_project_installed() -> bool {
    let project = std::path::Path::new("./src/frontend/package.json").exists();
    let installed = std::path::Path::new("./src/frontend/node_modules").exists();
    project && installed
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_node_installed() {
        // Test when node is installed and version is above required version
        assert_eq!(is_node_installed(), true);
    }

    #[test]
    fn test_is_frontend_project_installed() {
        assert_eq!(is_frontend_project_installed(), true);
    }
}
